use super::histogrammer::Histogrammer;
use polars::prelude::*;
use std::f64::consts::PI;

pub fn add_histograms(lf: LazyFrame) -> Result<Histogrammer, PolarsError> {
    let mut h = Histogrammer::new();

    let fp_bins = 600;
    let fp_range = (-300.0, 300.0);

    let caen_bins = 512;
    let caen_range = (0.0, 4096.0);

    let lf = lf.with_columns(vec![
        (col("DelayFrontRightEnergy") + col("DelayFrontLeftEnergy") / lit(2.0))
            .alias("DelayFrontAverageEnergy"),
        (col("DelayBackRightEnergy") + col("DelayBackLeftEnergy") / lit(2.0))
            .alias("DelayBackAverageEnergy"),
        (col("DelayFrontLeftTime") - col("AnodeFrontTime"))
            .alias("DelayFrontLeftTime_AnodeFrontTime"),
        (col("DelayFrontRightTime") - col("AnodeFrontTime"))
            .alias("DelayFrontRightTime_AnodeFrontTime"),
        (col("DelayBackLeftTime") - col("AnodeFrontTime"))
            .alias("DelayBackLeftTime_AnodeFrontTime"),
        (col("DelayBackRightTime") - col("AnodeFrontTime"))
            .alias("DelayBackRightTime_AnodeFrontTime"),
        (col("DelayFrontLeftTime") - col("AnodeBackTime"))
            .alias("DelayFrontLeftTime_AnodeBackTime"),
        (col("DelayFrontRightTime") - col("AnodeBackTime"))
            .alias("DelayFrontRightTime_AnodeBackTime"),
        (col("DelayBackLeftTime") - col("AnodeBackTime")).alias("DelayBackLeftTime_AnodeBackTime"),
        (col("DelayBackRightTime") - col("AnodeBackTime"))
            .alias("DelayBackRightTime_AnodeBackTime"),
        (col("AnodeFrontTime") - col("AnodeBackTime")).alias("AnodeFrontTime_AnodeBackTime"),
        (col("AnodeBackTime") - col("AnodeFrontTime")).alias("AnodeBackTime_AnodeFrontTime"),
        (col("AnodeFrontTime") - col("ScintLeftTime")).alias("AnodeFrontTime_ScintLeftTime"),
        (col("AnodeBackTime") - col("ScintLeftTime")).alias("AnodeBackTime_ScintLeftTime"),
        (col("DelayFrontLeftTime") - col("ScintLeftTime"))
            .alias("DelayFrontLeftTime_ScintLeftTime"),
        (col("DelayFrontRightTime") - col("ScintLeftTime"))
            .alias("DelayFrontRightTime_ScintLeftTime"),
        (col("DelayBackLeftTime") - col("ScintLeftTime")).alias("DelayBackLeftTime_ScintLeftTime"),
        (col("DelayBackRightTime") - col("ScintLeftTime"))
            .alias("DelayBackRightTime_ScintLeftTime"),
        (col("ScintRightTime") - col("ScintLeftTime")).alias("ScintRightTime_ScintLeftTime"),
    ]);

    h.add_fill_hist1d("Cebra0Energy", &lf, "Cebra0Energy", caen_bins, caen_range);
    h.add_fill_hist1d("Cebra1Energy", &lf, "Cebra1Energy", caen_bins, caen_range);
    h.add_fill_hist1d("Cebra2Energy", &lf, "Cebra2Energy", caen_bins, caen_range);
    h.add_fill_hist1d("Cebra3Energy", &lf, "Cebra3Energy", caen_bins, caen_range);
    h.add_fill_hist1d("Cebra4Energy", &lf, "Cebra4Energy", caen_bins, caen_range);

    h.add_fill_hist1d("X1", &lf, "X1", fp_bins, fp_range);
    h.add_fill_hist1d("X2", &lf, "X2", fp_bins, fp_range);
    h.add_fill_hist2d(
        "X2 v X1",
        &lf,
        "X1",
        "X2",
        (fp_bins, fp_bins),
        (fp_range, fp_range),
    );
    h.add_fill_hist2d(
        "DelayBackRight v X1",
        &lf,
        "X1",
        "DelayBackRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackLeft v X1",
        &lf,
        "X1",
        "DelayBackLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontRight v X1",
        &lf,
        "X1",
        "DelayFrontRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontLeft v X1",
        &lf,
        "X1",
        "DelayFrontLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackRight v X2",
        &lf,
        "X2",
        "DelayBackRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackLeft v X2",
        &lf,
        "X2",
        "DelayBackLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontRight v X2",
        &lf,
        "X2",
        "DelayFrontRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontLeft v X2",
        &lf,
        "X2",
        "DelayFrontLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackRight v Xavg",
        &lf,
        "Xavg",
        "DelayBackRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackLeft v Xavg",
        &lf,
        "Xavg",
        "DelayBackLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontRight v Xavg",
        &lf,
        "Xavg",
        "DelayFrontRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontLeft v Xavg",
        &lf,
        "Xavg",
        "DelayFrontLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontAverage v X1",
        &lf,
        "X1",
        "DelayFrontAverageEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackAverage v X1",
        &lf,
        "X1",
        "DelayBackAverageEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontAverage v X2",
        &lf,
        "X2",
        "DelayFrontAverageEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackAverage v X2",
        &lf,
        "X2",
        "DelayBackAverageEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayFrontAverage v Xavg",
        &lf,
        "Xavg",
        "DelayFrontAverageEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "DelayBackAverage v Xavg",
        &lf,
        "Xavg",
        "DelayBackAverageEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeBack v ScintLeft",
        &lf,
        "ScintLeftEnergy",
        "AnodeBackEnergy",
        (caen_bins, caen_bins),
        (caen_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeFront v ScintLeft",
        &lf,
        "ScintLeftEnergy",
        "AnodeFrontEnergy",
        (caen_bins, caen_bins),
        (caen_range, caen_range),
    );
    h.add_fill_hist2d(
        "Cathode v ScintLeft",
        &lf,
        "ScintLeftEnergy",
        "CathodeEnergy",
        (caen_bins, caen_bins),
        (caen_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeBack v ScintRight",
        &lf,
        "ScintRightEnergy",
        "AnodeBackEnergy",
        (caen_bins, caen_bins),
        (caen_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeFront v ScintRight",
        &lf,
        "ScintRightEnergy",
        "AnodeFrontEnergy",
        (caen_bins, caen_bins),
        (caen_range, caen_range),
    );
    h.add_fill_hist2d(
        "Cathode v ScintRight",
        &lf,
        "ScintRightEnergy",
        "CathodeEnergy",
        (caen_bins, caen_bins),
        (caen_range, caen_range),
    );
    h.add_fill_hist2d(
        "ScintLeft v X1",
        &lf,
        "X1",
        "ScintLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "ScintLeft v X2",
        &lf,
        "X2",
        "ScintLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "ScintLeft v Xavg",
        &lf,
        "Xavg",
        "ScintLeftEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "ScintRight v X1",
        &lf,
        "X1",
        "ScintRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "ScintRight v X2",
        &lf,
        "X2",
        "ScintRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "ScintRight v Xavg",
        &lf,
        "Xavg",
        "ScintRightEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeBack v X1",
        &lf,
        "X1",
        "AnodeBackEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeBack v X2",
        &lf,
        "X2",
        "AnodeBackEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeBack v Xavg",
        &lf,
        "Xavg",
        "AnodeBackEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeFront v X1",
        &lf,
        "X1",
        "AnodeFrontEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeFront v X2",
        &lf,
        "X2",
        "AnodeFrontEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "AnodeFront v Xavg",
        &lf,
        "Xavg",
        "AnodeFrontEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "Cathode v X1",
        &lf,
        "X1",
        "CathodeEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "Cathode v X2",
        &lf,
        "X2",
        "CathodeEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );
    h.add_fill_hist2d(
        "Cathode v Xavg",
        &lf,
        "Xavg",
        "CathodeEnergy",
        (fp_bins, caen_bins),
        (fp_range, caen_range),
    );

    // Both planes histograms
    let lf_bothplanes = lf
        .clone()
        .filter(col("X1").neq(lit(-1e6)))
        .filter(col("X2").neq(lit(-1e6)));

    h.add_fill_hist1d("X1: bothplanes", &lf_bothplanes, "X1", fp_bins, fp_range);
    h.add_fill_hist1d("X2: bothplanes", &lf_bothplanes, "X2", fp_bins, fp_range);

    h.add_fill_hist1d(
        "Xavg: bothplanes",
        &lf_bothplanes,
        "Xavg",
        fp_bins,
        fp_range,
    );

    h.add_fill_hist2d(
        "Theta v Xavg: bothplanes",
        &lf_bothplanes,
        "Xavg",
        "Theta",
        (fp_bins, 300),
        (fp_range, (0.0, PI / 2.0)),
    );
    h.add_fill_hist1d(
        "DelayFrontLeftTime_relTo_AnodeFrontTime_bothplanes",
        &lf_bothplanes,
        "DelayFrontLeftTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontRightTime_relTo_AnodeFrontTime_bothplanes",
        &lf_bothplanes,
        "DelayFrontRightTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackLeftTime_relTo_AnodeBackTime_bothplanes",
        &lf_bothplanes,
        "DelayBackLeftTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackRightTime_relTo_AnodeBackTime_bothplanes",
        &lf_bothplanes,
        "DelayBackRightTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );

    // Only 1 plane: X1
    let lf_only_x1_plane = lf
        .clone()
        .filter(col("X1").neq(lit(-1e6)))
        .filter(col("X2").eq(lit(-1e6)));

    h.add_fill_hist1d("X1: only1plane", &lf_only_x1_plane, "X1", fp_bins, fp_range);
    h.add_fill_hist1d(
        "DelayFrontLeftTime_relTo_AnodeFrontTime_noX2",
        &lf_only_x1_plane,
        "DelayFrontLeftTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontRightTime_relTo_AnodeFrontTime_noX2",
        &lf_only_x1_plane,
        "DelayFrontRightTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackLeftTime_relTo_AnodeFrontTime_noX2",
        &lf_only_x1_plane,
        "DelayBackLeftTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackRightTime_relTo_AnodeFrontTime_noX2",
        &lf_only_x1_plane,
        "DelayBackRightTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontLeftTime_relTo_AnodeBackTime_noX2",
        &lf_only_x1_plane,
        "DelayFrontLeftTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontRightTime_relTo_AnodeBackTime_noX2",
        &lf_only_x1_plane,
        "DelayFrontRightTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackLeftTime_relTo_AnodeBackTime_noX2",
        &lf_only_x1_plane,
        "DelayBackLeftTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackRightTime_relTo_AnodeBackTime_noX2",
        &lf_only_x1_plane,
        "DelayBackRightTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );

    // Only 1 plane: X2
    let lf_only_x2_plane = lf
        .clone()
        .filter(col("X2").neq(lit(-1e6)))
        .filter(col("X1").eq(lit(-1e6)));

    h.add_fill_hist1d("X2: only1plane", &lf_only_x2_plane, "X2", fp_bins, fp_range);
    h.add_fill_hist1d(
        "DelayFrontLeftTime_relTo_AnodeFrontTime_noX1",
        &lf_only_x2_plane,
        "DelayFrontLeftTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontRightTime_relTo_AnodeFrontTime_noX1",
        &lf_only_x2_plane,
        "DelayFrontRightTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackLeftTime_relTo_AnodeFrontTime_noX1",
        &lf_only_x2_plane,
        "DelayBackLeftTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackRightTime_relTo_AnodeFrontTime_noX1",
        &lf_only_x2_plane,
        "DelayBackRightTime_AnodeFrontTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontLeftTime_relTo_AnodeBackTime_noX1",
        &lf_only_x2_plane,
        "DelayFrontLeftTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontRightTime_relTo_AnodeBackTime_noX1",
        &lf_only_x2_plane,
        "DelayFrontRightTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackLeftTime_relTo_AnodeBackTime_noX1",
        &lf_only_x2_plane,
        "DelayBackLeftTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );
    h.add_fill_hist1d(
        "DelayBackRightTime_relTo_AnodeBackTime_noX1",
        &lf_only_x2_plane,
        "DelayBackRightTime_AnodeBackTime",
        8000,
        (-4000.0, 4000.0),
    );

    // Time relative to Back Anode

    let lf_time_rel_backanode = lf
        .clone()
        .filter(col("AnodeBackTime").neq(lit(-1e6)))
        .filter(col("ScintLeftTime").neq(lit(-1e6)));

    h.add_fill_hist1d(
        "AnodeFrontTime-AnodeBackTime",
        &lf_time_rel_backanode,
        "AnodeFrontTime_AnodeBackTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "AnodeBackTime-AnodeFrontTime",
        &lf_time_rel_backanode,
        "AnodeBackTime_AnodeFrontTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "AnodeFrontTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "AnodeFrontTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "AnodeBackTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "AnodeBackTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontLeftTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "DelayFrontLeftTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "DelayFrontRightTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "DelayFrontRightTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "DelayBackLeftTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "DelayBackLeftTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "DelayBackRightTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "DelayBackRightTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist1d(
        "ScintRightTime-ScintLeftTime",
        &lf_time_rel_backanode,
        "ScintRightTime_ScintLeftTime",
        1000,
        (-3000.0, 3000.0),
    );
    h.add_fill_hist2d(
        "ScintTimeDif v Xavg",
        &lf_time_rel_backanode,
        "Xavg",
        "ScintRightTime_ScintLeftTime",
        (fp_bins, 12800),
        (fp_range, (-3200.0, 3200.0)),
    );

    Ok(h)
}
