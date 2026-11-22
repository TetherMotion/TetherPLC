use sha2::{Digest, Sha256};
use std::process::ExitCode;
use tplc_pkg::format::SectionType;
use tplc_pkg::writer::PackageWriter;
/// `tplcc sign` — ed25519 package provenance (doc 15 §4.1).
///
///   tplcc sign keygen --dir <D> --name <N>   write N.key (sk) + <keyid>.pub
///   tplcc sign sign <in.tplc> --key <K.key> [-o <out.tplc>]
///   tplcc sign verify <pkg.tplc> [--trust-dir <D>]
///
/// Signature section payload: u32 ver(=1) | key_id[32] | pubkey[32] |
/// sig[64], where sig = ed25519.sk.sign(signing_hash) and
/// key_id = sha256(pubkey). signing_hash hashes non-signature TOC
/// entries (offset zeroed) + payloads, so it is stable when a section
/// is appended; signing an already-signed package replaces the
/// signature section.
pub(crate) fn cmd_sign(args: &[String]) -> ExitCode {
    use ed25519_dalek::{Signer, Verifier};

    let hex32 = |b: &[u8]| -> String { b.iter().map(|x| format!("{x:02x}")).collect() };
    let key_id_of = |pk: &[u8; 32]| -> [u8; 32] { Sha256::digest(pk).into() };

    if args.is_empty() {
        eprintln!("usage: tplcc sign <keygen|sign|verify> …");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "keygen" => {
            let mut dir = String::from(".");
            let mut name = String::from("key");
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--dir" => {
                        dir = args[i + 1].clone();
                        i += 2;
                    }
                    "--name" => {
                        name = args[i + 1].clone();
                        i += 2;
                    }
                    _ => {
                        eprintln!("sign keygen: bad arg {}", args[i]);
                        return ExitCode::from(2);
                    }
                }
            }
            if std::fs::create_dir_all(&dir).is_err() {
                eprintln!("sign keygen: mkdir {dir} failed");
                return ExitCode::from(1);
            }
            let mut sk_arr = [0u8; 32];
            rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut sk_arr);
            let sk = ed25519_dalek::SigningKey::from_bytes(&sk_arr);
            let pk = sk.verifying_key();
            let kid = key_id_of(pk.as_bytes());
            let kp = format!("{dir}/{name}.key");
            let pp = format!("{dir}/{}.pub", hex32(&kid));
            if std::fs::write(&kp, sk.to_bytes()).is_err()
                || std::fs::write(&pp, pk.to_bytes()).is_err()
            {
                eprintln!("sign keygen: write failed");
                return ExitCode::from(1);
            }
            println!("key_id {} → {kp} + {pp}", hex32(&kid));
            ExitCode::SUCCESS
        }
        "sign" => {
            let mut input: Option<String> = None;
            let mut key: Option<String> = None;
            let mut out: Option<String> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--key" => {
                        key = Some(args[i + 1].clone());
                        i += 2;
                    }
                    "-o" | "--out" => {
                        out = Some(args[i + 1].clone());
                        i += 2;
                    }
                    a if input.is_none() && !a.starts_with('-') => {
                        input = Some(a.to_string());
                        i += 1;
                    }
                    _ => {
                        eprintln!("sign sign: bad arg {}", args[i]);
                        return ExitCode::from(2);
                    }
                }
            }
            let (Some(input), Some(key)) = (input, key) else {
                eprintln!("usage: tplcc sign sign <in.tplc> --key <K.key> [-o out]");
                return ExitCode::from(2);
            };
            let out = out.unwrap_or_else(|| input.clone());
            let bytes = std::fs::read(&input).unwrap_or_else(|e| {
                eprintln!("{input}: {e}");
                std::process::exit(1)
            });
            let skb = std::fs::read(&key).unwrap_or_else(|e| {
                eprintln!("{key}: {e}");
                std::process::exit(1)
            });
            if skb.len() != 32 {
                eprintln!("{key}: expected 32-byte ed25519 secret");
                return ExitCode::from(1);
            }
            let sk_arr: [u8; 32] = skb[..32].try_into().unwrap();
            let sk = ed25519_dalek::SigningKey::from_bytes(&sk_arr);
            let pk = sk.verifying_key();
            let kid = key_id_of(pk.as_bytes());

            let pkg = tplc_pkg::Package::parse(&bytes).unwrap_or_else(|e| {
                eprintln!("{input}: parse: {e:?}");
                std::process::exit(1)
            });
            /* re-emit helper: copies every non-signature section */
            let emit = |sig_payload: Option<Vec<u8>>| -> Vec<u8> {
                let mut w = PackageWriter::new();
                w.uuid_raw(pkg.header.uuid);
                for s in pkg.sections() {
                    if s.section_type == SectionType::Signature as u32 {
                        continue; // replaced by the fresh signature
                    }
                    let _ = w.add_section_raw(
                        s.section_type,
                        s.data.to_vec(),
                        s.required(),
                        s.section_version,
                        s.aux,
                    );
                }
                if let Some(p) = sig_payload {
                    w.add_section(SectionType::Signature, p, false, 1, 0)
                        .unwrap_or_else(|e| {
                            eprintln!("sig section: {e:?}");
                            std::process::exit(1)
                        });
                }
                w.build().unwrap_or_else(|e| {
                    eprintln!("rebuild: {e:?}");
                    std::process::exit(1)
                })
            };
            let sig = sk.sign(&pkg.signing_hash());
            let mut payload = Vec::with_capacity(132);
            payload.extend_from_slice(&1u32.to_le_bytes());
            payload.extend_from_slice(&kid);
            payload.extend_from_slice(pk.as_bytes());
            payload.extend_from_slice(&sig.to_bytes());
            let signed = emit(Some(payload));
            std::fs::write(&out, &signed).unwrap_or_else(|e| {
                eprintln!("{out}: {e}");
                std::process::exit(1)
            });
            println!("signed {out} key_id {}", hex32(&kid));
            ExitCode::SUCCESS
        }
        "verify" => {
            let mut input: Option<String> = None;
            let mut trust_dir: Option<String> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--trust-dir" => {
                        trust_dir = Some(args[i + 1].clone());
                        i += 2;
                    }
                    a if input.is_none() && !a.starts_with('-') => {
                        input = Some(a.to_string());
                        i += 1;
                    }
                    _ => {
                        eprintln!("sign verify: bad arg {}", args[i]);
                        return ExitCode::from(2);
                    }
                }
            }
            let Some(input) = input else {
                eprintln!("usage: tplcc sign verify <pkg> [--trust-dir D]");
                return ExitCode::from(2);
            };
            let bytes = std::fs::read(&input).unwrap_or_else(|e| {
                eprintln!("{input}: {e}");
                std::process::exit(1)
            });
            let pkg = tplc_pkg::Package::parse(&bytes).unwrap_or_else(|e| {
                eprintln!("{input}: parse: {e:?}");
                std::process::exit(1)
            });
            let Some(sec) = pkg.section(SectionType::Signature) else {
                eprintln!("{input}: unsigned");
                return ExitCode::from(1);
            };
            let d = sec.data;
            if d.len() != 132 || u32::from_le_bytes(d[0..4].try_into().unwrap()) != 1 {
                eprintln!("{input}: bad signature section");
                return ExitCode::from(1);
            }
            let kid: &[u8; 32] = d[4..36].try_into().unwrap();
            let pk_bytes: &[u8; 32] = d[36..68].try_into().unwrap();
            if &key_id_of(pk_bytes) != kid {
                eprintln!("{input}: key_id/pubkey mismatch");
                return ExitCode::from(1);
            }
            let pk = ed25519_dalek::VerifyingKey::from_bytes(pk_bytes).unwrap();
            let sig = ed25519_dalek::Signature::from_bytes(d[68..132].try_into().unwrap());
            if pk.verify(&pkg.signing_hash(), &sig).is_err() {
                eprintln!("{input}: signature invalid");
                return ExitCode::from(1);
            }
            if let Some(td) = trust_dir {
                let f = format!("{td}/{}.pub", hex32(kid));
                match std::fs::read(&f) {
                    Ok(b) if b == pk_bytes => {}
                    Ok(_) => {
                        eprintln!("{input}: trust store key mismatch");
                        return ExitCode::from(1);
                    }
                    Err(e) => {
                        eprintln!("{input}: untrusted signer ({f}: {e})");
                        return ExitCode::from(1);
                    }
                }
            }
            println!("{input}: signature OK key_id {}", hex32(kid));
            ExitCode::SUCCESS
        }
        c => {
            eprintln!("tplcc sign: unknown subcommand {c}");
            ExitCode::from(2)
        }
    }
}

