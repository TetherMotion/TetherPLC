/* iec61850_ied_fake.cpp — minimal IEC 61850 IED server for the
 * tplc-iec61850 e2e.  Uses libiec61850's IedServer + CDC model API to
 * expose a small Logical Device / Logical Node tree the client side
 * reads and writes:
 *
 *   domain (LD):    "tetherTestLD"
 *   nodes:          LLN0.Health.stVal  (int32 status, R/W under ST)
 *                   LLN0.Meas.mag.f    (float measurement, R/W under MX)
 *                   LLN0.Name.name     (string, read)
 *
 * Wire-level peer for Iec61850Service — real MMS, not a stub. */
#include <iec61850_server.h>
#include <iec61850_model.h>
#include <iec61850_dynamic_model.h>
#include <iec61850_cdc.h>
#include <hal_thread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>

static volatile int stopFlag = 0;
static void onSig(int) { stopFlag = 1; }

static IedServer g_ied;
static DataAttribute* g_swi_stval;   /* valWTr.posVal leaf */

/* BSC control handler: accept the command and mirror ctlVal into
   valWTr.posVal so clients can verify the operate took effect. */
static ControlHandlerResult
swiControlHandler(ControlAction action, void* parameter,
                  MmsValue* value, bool test)
{
    (void)parameter;
    if (test || !value) return CONTROL_RESULT_FAILED;
    bool on;
    switch (MmsValue_getType(value)) {
        case MMS_BOOLEAN: on = MmsValue_getBoolean(value); break;
        case MMS_BIT_STRING: on = MmsValue_getBitStringAsInteger(value) != 0; break;
        case MMS_INTEGER:
        case MMS_UNSIGNED: on = MmsValue_toInt64(value) != 0; break;
        case MMS_FLOAT: on = MmsValue_toFloat(value) != 0.0f; break;
        default: return CONTROL_RESULT_FAILED;
    }
    printf("SWI1 control: %s\n", on ? "ON" : "OFF");
    fflush(stdout);
    IedServer_lockDataModel(g_ied);
    IedServer_updateInt32AttributeValue(g_ied, g_swi_stval,
                                        on ? 1 : 0);
    IedServer_unlockDataModel(g_ied);
    return CONTROL_RESULT_OK;
}

int main(int argc, char** argv) {
    int port = argc > 1 ? atoi(argv[1]) : 1102;

    IedModel* model = IedModel_create("tetherIED");
    LogicalDevice* ld = LogicalDevice_create("tetherTestLD", model);
    LogicalNode* lln0 = LogicalNode_create("LLN0", ld);

    /* Health: enum status (stVal int32) R/W under ST. */
    CDC_ENS_create("Health", (ModelNode*)lln0, 0);
    /* Meas: sampled analog value (instMag.f float) under MX. */
    CDC_SAV_create("Meas", (ModelNode*)lln0, 0, false);
    /* SetPt: analogue setting (setMag.f float) R/W under SP. */
    CDC_ASG_create("SetPt", (ModelNode*)lln0, 0, false);
    /* SetPt2: second writable setting for the WriteDataSet e2e
       (Health.stVal is read-only). */
    CDC_ASG_create("SetPt2", (ModelNode*)lln0, 0, false);
    /* Name: visible string (read). */
    CDC_VSS_create("Name", (ModelNode*)lln0, 0);

    /* seed values — ModelNode_getChild takes the DO then the DA name. */
    ModelNode* health = ModelNode_getChild((ModelNode*)lln0, "Health");
    if (health) {
        DataAttribute* stVal = (DataAttribute*)ModelNode_getChild(health, "stVal");
        if (stVal) DataAttribute_setValue(stVal, MmsValue_newInteger(7));
    }
    ModelNode* meas = ModelNode_getChild((ModelNode*)lln0, "Meas");
    if (meas) {
        DataAttribute* mag = (DataAttribute*)ModelNode_getChild(meas, "instMag.f");
        if (mag) DataAttribute_setValue(mag, MmsValue_newFloat(42.5f));
    }
    ModelNode* setpt = ModelNode_getChild((ModelNode*)lln0, "SetPt");
    if (setpt) {
        DataAttribute* sm = (DataAttribute*)ModelNode_getChild(setpt, "setMag.f");
        if (sm) DataAttribute_setValue(sm, MmsValue_newFloat(1.0f));
    }
    ModelNode* setpt2 = ModelNode_getChild((ModelNode*)lln0, "SetPt2");
    if (setpt2) {
        DataAttribute* sm2 = (DataAttribute*)ModelNode_getChild(setpt2, "setMag.f");
        if (sm2) DataAttribute_setValue(sm2, MmsValue_newFloat(2.0f));
    }
    ModelNode* name = ModelNode_getChild((ModelNode*)lln0, "Name");
    if (name) {
        DataAttribute* nm = (DataAttribute*)ModelNode_getChild(name, "val");
        if (nm) DataAttribute_setValue(nm, MmsValue_newVisibleString("TetherIED"));
    }

    /* RCB surface: a data set over Meas.instMag.f + Health.stVal and
       an unbuffered report control block (URCB, FC RP).  Clients
       enable it via SetRCBValues; data changes then trigger dchg
       InformationReports. */
    DataSet* ds = DataSet_create("measds", lln0);
    DataSetEntry_create(ds, "LLN0$MX$Meas$instMag$f", -1, NULL);
    DataSetEntry_create(ds, "LLN0$SP$SetPt$setMag$f", -1, NULL);
    ReportControlBlock_create("measurcb", lln0, "rptMeas", false,
                              "measds", 1,
                              TRG_OPT_DATA_CHANGED | TRG_OPT_GI,
                              RPT_OPT_SEQ_NUM | RPT_OPT_REASON_FOR_INCLUSION |
                              RPT_OPT_DATA_SET |
                              RPT_OPT_TIME_STAMP,
                              0, 0);
    /* buffered twin: same dataset, BR reference, 50 ms bufTm and an
       200 ms integrity period (TRG_OPT_INTEGRITY). */
    ReportControlBlock_create("bufrcb", lln0, "rptBuf", true,
                              "measds", 1,
                              TRG_OPT_DATA_CHANGED | TRG_OPT_GI |
                              TRG_OPT_INTEGRITY,
                              RPT_OPT_SEQ_NUM | RPT_OPT_REASON_FOR_INCLUSION |
                              RPT_OPT_DATA_SET |
                              RPT_OPT_TIME_STAMP,
                              50, 200);
    /* writable twin for WriteDataSet e2e: ST + SP members only
       (instMag.f under MX is read-only and would fail the write). */
    DataSet* wds = DataSet_create("writeds", lln0);
    DataSetEntry_create(wds, "LLN0$SP$SetPt$setMag$f", -1, NULL);
    DataSetEntry_create(wds, "LLN0$SP$SetPt2$setMag$f", -1, NULL);

    /* SWI1: controllable single point (BSC), SBO normal security with
       cancel — exercises select/operate/cancel over the CO model. */
    DataObject* swi1 = CDC_BSC_create("SWI1", (ModelNode*)lln0, 0,
                                    CDC_CTL_MODEL_SBO_NORMAL |
                                    CDC_CTL_MODEL_HAS_CANCEL, false);
    if (swi1) {
        ModelNode* vwt = ModelNode_getChild((ModelNode*)swi1, "valWTr");
        ModelNode* pv = vwt ? ModelNode_getChild(vwt, "posVal") : NULL;
        if (pv) {
            g_swi_stval = (DataAttribute*)pv;
            DataAttribute_setValue(g_swi_stval,
                MmsValue_newInteger(0));
        }
    }

    IedServer ied = IedServer_create(model);
    g_ied = ied;
    if (swi1)
        IedServer_setControlHandler(ied, swi1, swiControlHandler, NULL);
    IedServer_setLocalIpAddress(ied, "127.0.0.1");
    IedServer_start(ied, port);

    printf("iec61850_ied_fake on %d\n", port);
    fflush(stdout);

    signal(SIGINT, onSig);
    signal(SIGTERM, onSig);
    while (!stopFlag)
        Thread_sleep(200);

    IedServer_stop(ied);
    IedServer_destroy(ied);
    return 0;
}
