package org.firstinspires.ftc.teamcode;

import com.qualcomm.robotcore.eventloop.opmode.LinearOpMode;
import com.qualcomm.robotcore.eventloop.opmode.TeleOp;
import com.qualcomm.robotcore.hardware.HardwareDevice;
import com.qualcomm.robotcore.hardware.HardwareMap;

@TeleOp(name="Rust")
public class RustTest extends LinearOpMode {
	private static native int multiply(int a, int b);

	static {
		System.loadLibrary("rust");
	}

	@Override
	public void runOpMode() throws InterruptedException {
		telemetry.addData("Status", "Initialized");
		telemetry.update();
		waitForStart();

		telemetry.addData("Status", multiply(2, 3));
		telemetry.update();
		while (opModeIsActive()) {
			idle();
		}
	}
}
