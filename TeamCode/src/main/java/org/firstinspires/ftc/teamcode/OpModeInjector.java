package org.firstinspires.ftc.teamcode;

import com.qualcomm.robotcore.eventloop.opmode.LinearOpMode;
import com.qualcomm.robotcore.eventloop.opmode.OpModeManager;
import com.qualcomm.robotcore.eventloop.opmode.OpModeRegistrar;

import org.firstinspires.ftc.robotcore.internal.opmode.OpModeMeta;

public class OpModeInjector {
	@OpModeRegistrar
	public static void injectOpModes(OpModeManager manager) {
		OpModeMeta metaData = new OpModeMeta.Builder()
				.setSource(OpModeMeta.Source.ANDROID_STUDIO)
				.setFlavor(OpModeMeta.Flavor.TELEOP)
				.setName("experimental opmode")
				.setGroup("test")
				.build();

		LinearOpMode experimental = new LinearOpMode() {
			@Override
			public void runOpMode() throws InterruptedException {
				telemetry.addData("Status", "Initialized");
				telemetry.update();

				waitForStart();

				while (opModeIsActive()) {
					telemetry.addData("Status", "Running");
					telemetry.update();
				}
			}
		};

		manager.register(metaData, experimental);
	}
}
