// exports from <IOKit/hid/IOHIDUsageTables.h>

// Usage Pages
pub const kHIDPage_Undefined: u32 = 0x00;
pub const kHIDPage_GenericDesktop: u32 = 0x01;
pub const kHIDPage_Simulation: u32 = 0x02;
pub const kHIDPage_VR: u32 = 0x03;
pub const kHIDPage_Sport: u32 = 0x04;
pub const kHIDPage_Game: u32 = 0x05;
pub const kHIDPage_GenericDeviceControls: u32 = 0x06;
pub const kHIDPage_KeyboardOrKeypad: u32 = 0x07;
pub const kHIDPage_LEDs: u32 = 0x08;
pub const kHIDPage_Button: u32 = 0x09;
pub const kHIDPage_Ordinal: u32 = 0x0A;
pub const kHIDPage_Telephony: u32 = 0x0B;
pub const kHIDPage_Consumer: u32 = 0x0C;
pub const kHIDPage_Digitizer: u32 = 0x0D;
// Reserved 0x0E
pub const kHIDPage_PID: u32 = 0x0F;
pub const kHIDPage_Unicode: u32 = 0x10;
// Reserved 0x11 - 0x13
pub const kHIDPage_AlphanumericDisplay: u32 = 0x14;
// Reserved 0x15 - 0x1F
pub const kHIDPage_Sensor: u32 = 0x20;
// Reserved 0x21 - 0x7f
pub const kHIDPage_Monitor: u32 = 0x80;
pub const kHIDPage_MonitorEnumerated: u32 = 0x81;
pub const kHIDPage_MonitorVirtual: u32 = 0x82;
pub const kHIDPage_MonitorReserved: u32 = 0x83;
// Power 0x84 - 0x87     USB Device Class Definition for Power Devices
pub const kHIDPage_PowerDevice: u32 = 0x84;
pub const kHIDPage_BatterySystem: u32 = 0x85;
pub const kHIDPage_PowerReserved: u32 = 0x86;
pub const kHIDPage_PowerReserved2: u32 = 0x87;
// Reserved 0x88 - 0x8B
pub const kHIDPage_BarCodeScanner: u32 = 0x8C;
pub const kHIDPage_WeighingDevice: u32 = 0x8D;
pub const kHIDPage_Scale: u32 = 0x8D;
pub const kHIDPage_MagneticStripeReader: u32 = 0x8E;
// ReservedPointofSalepages 0x8F
pub const kHIDPage_CameraControl: u32 = 0x90;
pub const kHIDPage_Arcade: u32 = 0x91;
// Reserved 0x92 - 0xFEFF
// VendorDefined 0xFF00 - 0xFFFF
pub const kHIDPage_VendorDefinedStart: u32 = 0xFF00;

// Undefined Usage for all usage pages
pub const kHIDUsage_Undefined: u32 = 0x00;

// GenericDesktop Page (0x01)
pub const kHIDUsage_GD_Pointer: u32 = 0x01;
pub const kHIDUsage_GD_Mouse: u32 = 0x02;
// 0x03 Reserved
pub const kHIDUsage_GD_Joystick: u32 = 0x04;
pub const kHIDUsage_GD_GamePad: u32 = 0x05;
pub const kHIDUsage_GD_Keyboard: u32 = 0x06;
pub const kHIDUsage_GD_Keypad: u32 = 0x07;
pub const kHIDUsage_GD_MultiAxisController: u32 = 0x08;
// 0x09 - 0x2F Reserved
pub const kHIDUsage_GD_X: u32 = 0x30;
pub const kHIDUsage_GD_Y: u32 = 0x31;
pub const kHIDUsage_GD_Z: u32 = 0x32;
pub const kHIDUsage_GD_Rx: u32 = 0x33;
pub const kHIDUsage_GD_Ry: u32 = 0x34;
pub const kHIDUsage_GD_Rz: u32 = 0x35;
pub const kHIDUsage_GD_Slider: u32 = 0x36;
pub const kHIDUsage_GD_Dial: u32 = 0x37;
pub const kHIDUsage_GD_Wheel: u32 = 0x38;
pub const kHIDUsage_GD_Hatswitch: u32 = 0x39;
pub const kHIDUsage_GD_CountedBuffer: u32 = 0x3A;
pub const kHIDUsage_GD_ByteCount: u32 = 0x3B;
pub const kHIDUsage_GD_MotionWakeup: u32 = 0x3C;
pub const kHIDUsage_GD_Start: u32 = 0x3D;
pub const kHIDUsage_GD_Select: u32 = 0x3E;
// 0x3F Reserved
pub const kHIDUsage_GD_Vx: u32 = 0x40;
pub const kHIDUsage_GD_Vy: u32 = 0x41;
pub const kHIDUsage_GD_Vz: u32 = 0x42;
pub const kHIDUsage_GD_Vbrx: u32 = 0x43;
pub const kHIDUsage_GD_Vbry: u32 = 0x44;
pub const kHIDUsage_GD_Vbrz: u32 = 0x45;
pub const kHIDUsage_GD_Vno: u32 = 0x46;
// 0x47 - 0x7F Reserved
pub const kHIDUsage_GD_SystemControl: u32 = 0x80;
pub const kHIDUsage_GD_SystemPowerDown: u32 = 0x81;
pub const kHIDUsage_GD_SystemSleep: u32 = 0x82;
pub const kHIDUsage_GD_SystemWakeUp: u32 = 0x83;
pub const kHIDUsage_GD_SystemContextMenu: u32 = 0x84;
pub const kHIDUsage_GD_SystemMainMenu: u32 = 0x85;
pub const kHIDUsage_GD_SystemAppMenu: u32 = 0x86;
pub const kHIDUsage_GD_SystemMenuHelp: u32 = 0x87;
pub const kHIDUsage_GD_SystemMenuExit: u32 = 0x88;
pub const kHIDUsage_GD_SystemMenuSelect: u32 = 0x89;
pub const kHIDUsage_GD_SystemMenu: u32 = kHIDUsage_GD_SystemMenuSelect;
pub const kHIDUsage_GD_SystemMenuRight: u32 = 0x8A;
pub const kHIDUsage_GD_SystemMenuLeft: u32 = 0x8B;
pub const kHIDUsage_GD_SystemMenuUp: u32 = 0x8C;
pub const kHIDUsage_GD_SystemMenuDown: u32 = 0x8D;
// 0x8E - 0x8F Reserved
pub const kHIDUsage_GD_DPadUp: u32 = 0x90;
pub const kHIDUsage_GD_DPadDown: u32 = 0x91;
pub const kHIDUsage_GD_DPadRight: u32 = 0x92;
pub const kHIDUsage_GD_DPadLeft: u32 = 0x93;
// 0x94 - 0xFFFF Reserved
pub const kHIDUsage_GD_Reserved: u32 = 0xFFFF;

// Simulation Page (0x02)
pub const kHIDUsage_Sim_FlightSimulationDevice: u32 = 0x01;
pub const kHIDUsage_Sim_AutomobileSimulationDevice: u32 = 0x02;
pub const kHIDUsage_Sim_TankSimulationDevice: u32 = 0x03;
pub const kHIDUsage_Sim_SpaceshipSimulationDevice: u32 = 0x04;
pub const kHIDUsage_Sim_SubmarineSimulationDevice: u32 = 0x05;
pub const kHIDUsage_Sim_SailingSimulationDevice: u32 = 0x06;
pub const kHIDUsage_Sim_MotorcycleSimulationDevice: u32 = 0x07;
pub const kHIDUsage_Sim_SportsSimulationDevice: u32 = 0x08;
pub const kHIDUsage_Sim_AirplaneSimulationDevice: u32 = 0x09;
pub const kHIDUsage_Sim_HelicopterSimulationDevice: u32 = 0x0A;
pub const kHIDUsage_Sim_MagicCarpetSimulationDevice: u32 = 0x0B;
pub const kHIDUsage_Sim_BicycleSimulationDevice: u32 = 0x0C;
// 0x0D - 0x1F Reserved
pub const kHIDUsage_Sim_FlightControlStick: u32 = 0x20;
pub const kHIDUsage_Sim_FlightStick: u32 = 0x21;
pub const kHIDUsage_Sim_CyclicControl: u32 = 0x22;
pub const kHIDUsage_Sim_CyclicTrim: u32 = 0x23;
pub const kHIDUsage_Sim_FlightYoke: u32 = 0x24;
pub const kHIDUsage_Sim_TrackControl: u32 = 0x25;
// 0x26 - 0xAF Reserved
pub const kHIDUsage_Sim_Aileron: u32 = 0xB0;
pub const kHIDUsage_Sim_AileronTrim: u32 = 0xB1;
pub const kHIDUsage_Sim_AntiTorqueControl: u32 = 0xB2;
pub const kHIDUsage_Sim_AutopilotEnable: u32 = 0xB3;
pub const kHIDUsage_Sim_ChaffRelease: u32 = 0xB4;
pub const kHIDUsage_Sim_CollectiveControl: u32 = 0xB5;
pub const kHIDUsage_Sim_DiveBrake: u32 = 0xB6;
pub const kHIDUsage_Sim_ElectronicCountermeasures: u32 = 0xB7;
pub const kHIDUsage_Sim_Elevator: u32 = 0xB8;
pub const kHIDUsage_Sim_ElevatorTrim: u32 = 0xB9;
pub const kHIDUsage_Sim_Rudder: u32 = 0xBA;
pub const kHIDUsage_Sim_Throttle: u32 = 0xBB;
pub const kHIDUsage_Sim_FlightCommunications: u32 = 0xBC;
pub const kHIDUsage_Sim_FlareRelease: u32 = 0xBD;
pub const kHIDUsage_Sim_LandingGear: u32 = 0xBE;
pub const kHIDUsage_Sim_ToeBrake: u32 = 0xBF;
pub const kHIDUsage_Sim_Trigger: u32 = 0xC0;
pub const kHIDUsage_Sim_WeaponsArm: u32 = 0xC1;
pub const kHIDUsage_Sim_Weapons: u32 = 0xC2;
pub const kHIDUsage_Sim_WingFlaps: u32 = 0xC3;
pub const kHIDUsage_Sim_Accelerator: u32 = 0xC4;
pub const kHIDUsage_Sim_Brake: u32 = 0xC5;
pub const kHIDUsage_Sim_Clutch: u32 = 0xC6;
pub const kHIDUsage_Sim_Shifter: u32 = 0xC7;
pub const kHIDUsage_Sim_Steering: u32 = 0xC8;
pub const kHIDUsage_Sim_TurretDirection: u32 = 0xC9;
pub const kHIDUsage_Sim_BarrelElevation: u32 = 0xCA;
pub const kHIDUsage_Sim_DivePlane: u32 = 0xCB;
pub const kHIDUsage_Sim_Ballast: u32 = 0xCC;
pub const kHIDUsage_Sim_BicycleCrank: u32 = 0xCD;
pub const kHIDUsage_Sim_HandleBars: u32 = 0xCE;
pub const kHIDUsage_Sim_FrontBrake: u32 = 0xCF;
pub const kHIDUsage_Sim_RearBrake: u32 = 0xD0;
// 0xD1 - 0xFFFF Reserved
pub const kHIDUsage_Sim_Reserved: u32 = 0xFFFF;

// VR Page (0x03)
pub const kHIDUsage_VR_Belt: u32 = 0x01;
pub const kHIDUsage_VR_BodySuit: u32 = 0x02;
pub const kHIDUsage_VR_Flexor: u32 = 0x03;
pub const kHIDUsage_VR_Glove: u32 = 0x04;
pub const kHIDUsage_VR_HeadTracker: u32 = 0x05;
pub const kHIDUsage_VR_HeadMountedDisplay: u32 = 0x06;
pub const kHIDUsage_VR_HandTracker: u32 = 0x07;
pub const kHIDUsage_VR_Oculometer: u32 = 0x08;
pub const kHIDUsage_VR_Vest: u32 = 0x09;
pub const kHIDUsage_VR_AnimatronicDevice: u32 = 0x0A;
// 0x0B - 0x1F Reserved
pub const kHIDUsage_VR_StereoEnable: u32 = 0x20;
pub const kHIDUsage_VR_DisplayEnable: u32 = 0x21;
// 0x22 - 0xFFFF Reserved
pub const kHIDUsage_VR_Reserved: u32 = 0xFFFF;

// Sport Page (0x04)
pub const kHIDUsage_Sprt_BaseballBat: u32 = 0x01;
pub const kHIDUsage_Sprt_GolfClub: u32 = 0x02;
pub const kHIDUsage_Sprt_RowingMachine: u32 = 0x03;
pub const kHIDUsage_Sprt_Treadmill: u32 = 0x04;
// 0x05 - 0x2F Reserved
pub const kHIDUsage_Sprt_Oar: u32 = 0x30;
pub const kHIDUsage_Sprt_Slope: u32 = 0x31;
pub const kHIDUsage_Sprt_Rate: u32 = 0x32;
pub const kHIDUsage_Sprt_StickSpeed: u32 = 0x33;
pub const kHIDUsage_Sprt_StickFaceAngle: u32 = 0x34;
pub const kHIDUsage_Sprt_StickHeelOrToe: u32 = 0x35;
pub const kHIDUsage_Sprt_StickFollowThrough: u32 = 0x36;
pub const kHIDUsage_Sprt_StickTempo: u32 = 0x37;
pub const kHIDUsage_Sprt_StickType: u32 = 0x38;
pub const kHIDUsage_Sprt_StickHeight: u32 = 0x39;
// 0x3A - 0x4F Reserved
pub const kHIDUsage_Sprt_Putter: u32 = 0x50;
pub const kHIDUsage_Sprt_1Iron: u32 = 0x51;
pub const kHIDUsage_Sprt_2Iron: u32 = 0x52;
pub const kHIDUsage_Sprt_3Iron: u32 = 0x53;
pub const kHIDUsage_Sprt_4Iron: u32 = 0x54;
pub const kHIDUsage_Sprt_5Iron: u32 = 0x55;
pub const kHIDUsage_Sprt_6Iron: u32 = 0x56;
pub const kHIDUsage_Sprt_7Iron: u32 = 0x57;
pub const kHIDUsage_Sprt_8Iron: u32 = 0x58;
pub const kHIDUsage_Sprt_9Iron: u32 = 0x59;
pub const kHIDUsage_Sprt_10Iron: u32 = 0x5A;
pub const kHIDUsage_Sprt_11Iron: u32 = 0x5B;
pub const kHIDUsage_Sprt_SandWedge: u32 = 0x5C;
pub const kHIDUsage_Sprt_LoftWedge: u32 = 0x5D;
pub const kHIDUsage_Sprt_PowerWedge: u32 = 0x5E;
pub const kHIDUsage_Sprt_1Wood: u32 = 0x5F;
pub const kHIDUsage_Sprt_3Wood: u32 = 0x60;
pub const kHIDUsage_Sprt_5Wood: u32 = 0x61;
pub const kHIDUsage_Sprt_7Wood: u32 = 0x62;
pub const kHIDUsage_Sprt_9Wood: u32 = 0x63;
// 0x64 - 0xFFFF Reserved
pub const kHIDUsage_Sprt_Reserved: u32 = 0xFFFF;

// Game Page (0x05)
pub const kHIDUsage_Game_3DGameController: u32 = 0x01;
pub const kHIDUsage_Game_PinballDevice: u32 = 0x02;
pub const kHIDUsage_Game_GunDevice: u32 = 0x03;
// 0x04 - 0x1F Reserved
pub const kHIDUsage_Game_PointofView: u32 = 0x20;
pub const kHIDUsage_Game_TurnRightOrLeft: u32 = 0x21;
pub const kHIDUsage_Game_PitchUpOrDown: u32 = 0x22;
pub const kHIDUsage_Game_RollRightOrLeft: u32 = 0x23;
pub const kHIDUsage_Game_MoveRightOrLeft: u32 = 0x24;
pub const kHIDUsage_Game_MoveForwardOrBackward: u32 = 0x25;
pub const kHIDUsage_Game_MoveUpOrDown: u32 = 0x26;
pub const kHIDUsage_Game_LeanRightOrLeft: u32 = 0x27;
pub const kHIDUsage_Game_LeanForwardOrBackward: u32 = 0x28;
pub const kHIDUsage_Game_HeightOfPOV: u32 = 0x29;
pub const kHIDUsage_Game_Flipper: u32 = 0x2A;
pub const kHIDUsage_Game_SecondaryFlipper: u32 = 0x2B;
pub const kHIDUsage_Game_Bump: u32 = 0x2C;
pub const kHIDUsage_Game_NewGame: u32 = 0x2D;
pub const kHIDUsage_Game_ShootBall: u32 = 0x2E;
pub const kHIDUsage_Game_Player: u32 = 0x2F;
pub const kHIDUsage_Game_GunBolt: u32 = 0x30;
pub const kHIDUsage_Game_GunClip: u32 = 0x31;
pub const kHIDUsage_Game_Gun: u32 = 0x32;
pub const kHIDUsage_Game_GunSingleShot: u32 = 0x33;
pub const kHIDUsage_Game_GunBurst: u32 = 0x34;
pub const kHIDUsage_Game_GunAutomatic: u32 = 0x35;
pub const kHIDUsage_Game_GunSafety: u32 = 0x36;
pub const kHIDUsage_Game_GamepadFireOrJump: u32 = 0x37;
pub const kHIDUsage_Game_GamepadTrigger: u32 = 0x39;
// 0x3A - 0xFFFF Reserved
pub const kHIDUsage_Game_Reserved: u32 = 0xFFFF;

// Generic Device Controls (0x0g)
pub const kHIDUsage_GenDevControls_BackgroundControls: u32 = 0x01;

// KeyboardOrKeypad Page (0x07)
pub const kHIDUsage_KeyboardErrorRollOver: u32 = 0x01;
pub const kHIDUsage_KeyboardPOSTFail: u32 = 0x02;
pub const kHIDUsage_KeyboardErrorUndefined: u32 = 0x03;
pub const kHIDUsage_KeyboardA: u32 = 0x04;
pub const kHIDUsage_KeyboardB: u32 = 0x05;
pub const kHIDUsage_KeyboardC: u32 = 0x06;
pub const kHIDUsage_KeyboardD: u32 = 0x07;
pub const kHIDUsage_KeyboardE: u32 = 0x08;
pub const kHIDUsage_KeyboardF: u32 = 0x09;
pub const kHIDUsage_KeyboardG: u32 = 0x0A;
pub const kHIDUsage_KeyboardH: u32 = 0x0B;
pub const kHIDUsage_KeyboardI: u32 = 0x0C;
pub const kHIDUsage_KeyboardJ: u32 = 0x0D;
pub const kHIDUsage_KeyboardK: u32 = 0x0E;
pub const kHIDUsage_KeyboardL: u32 = 0x0F;
pub const kHIDUsage_KeyboardM: u32 = 0x10;
pub const kHIDUsage_KeyboardN: u32 = 0x11;
pub const kHIDUsage_KeyboardO: u32 = 0x12;
pub const kHIDUsage_KeyboardP: u32 = 0x13;
pub const kHIDUsage_KeyboardQ: u32 = 0x14;
pub const kHIDUsage_KeyboardR: u32 = 0x15;
pub const kHIDUsage_KeyboardS: u32 = 0x16;
pub const kHIDUsage_KeyboardT: u32 = 0x17;
pub const kHIDUsage_KeyboardU: u32 = 0x18;
pub const kHIDUsage_KeyboardV: u32 = 0x19;
pub const kHIDUsage_KeyboardW: u32 = 0x1A;
pub const kHIDUsage_KeyboardX: u32 = 0x1B;
pub const kHIDUsage_KeyboardY: u32 = 0x1C;
pub const kHIDUsage_KeyboardZ: u32 = 0x1D;
pub const kHIDUsage_Keyboard1: u32 = 0x1E;
pub const kHIDUsage_Keyboard2: u32 = 0x1F;
pub const kHIDUsage_Keyboard3: u32 = 0x20;
pub const kHIDUsage_Keyboard4: u32 = 0x21;
pub const kHIDUsage_Keyboard5: u32 = 0x22;
pub const kHIDUsage_Keyboard6: u32 = 0x23;
pub const kHIDUsage_Keyboard7: u32 = 0x24;
pub const kHIDUsage_Keyboard8: u32 = 0x25;
pub const kHIDUsage_Keyboard9: u32 = 0x26;
pub const kHIDUsage_Keyboard0: u32 = 0x27;
pub const kHIDUsage_KeyboardReturnOrEnter: u32 = 0x28;
pub const kHIDUsage_KeyboardEscape: u32 = 0x29;
pub const kHIDUsage_KeyboardDeleteOrBackspace: u32 = 0x2A;
pub const kHIDUsage_KeyboardTab: u32 = 0x2B;
pub const kHIDUsage_KeyboardSpacebar: u32 = 0x2C;
pub const kHIDUsage_KeyboardHyphen: u32 = 0x2D;
pub const kHIDUsage_KeyboardEqualSign: u32 = 0x2E;
pub const kHIDUsage_KeyboardOpenBracket: u32 = 0x2F;
pub const kHIDUsage_KeyboardCloseBracket: u32 = 0x30;
pub const kHIDUsage_KeyboardBackslash: u32 = 0x31;
pub const kHIDUsage_KeyboardNonUSPound: u32 = 0x32;
pub const kHIDUsage_KeyboardSemicolon: u32 = 0x33;
pub const kHIDUsage_KeyboardQuote: u32 = 0x34;
pub const kHIDUsage_KeyboardGraveAccentAndTilde: u32 = 0x35;
pub const kHIDUsage_KeyboardComma: u32 = 0x36;
pub const kHIDUsage_KeyboardPeriod: u32 = 0x37;
pub const kHIDUsage_KeyboardSlash: u32 = 0x38;
pub const kHIDUsage_KeyboardCapsLock: u32 = 0x39;
pub const kHIDUsage_KeyboardF1: u32 = 0x3A;
pub const kHIDUsage_KeyboardF2: u32 = 0x3B;
pub const kHIDUsage_KeyboardF3: u32 = 0x3C;
pub const kHIDUsage_KeyboardF4: u32 = 0x3D;
pub const kHIDUsage_KeyboardF5: u32 = 0x3E;
pub const kHIDUsage_KeyboardF6: u32 = 0x3F;
pub const kHIDUsage_KeyboardF7: u32 = 0x40;
pub const kHIDUsage_KeyboardF8: u32 = 0x41;
pub const kHIDUsage_KeyboardF9: u32 = 0x42;
pub const kHIDUsage_KeyboardF10: u32 = 0x43;
pub const kHIDUsage_KeyboardF11: u32 = 0x44;
pub const kHIDUsage_KeyboardF12: u32 = 0x45;
pub const kHIDUsage_KeyboardPrintScreen: u32 = 0x46;
pub const kHIDUsage_KeyboardScrollLock: u32 = 0x47;
pub const kHIDUsage_KeyboardPause: u32 = 0x48;
pub const kHIDUsage_KeyboardInsert: u32 = 0x49;
pub const kHIDUsage_KeyboardHome: u32 = 0x4A;
pub const kHIDUsage_KeyboardPageUp: u32 = 0x4B;
pub const kHIDUsage_KeyboardDeleteForward: u32 = 0x4C;
pub const kHIDUsage_KeyboardEnd: u32 = 0x4D;
pub const kHIDUsage_KeyboardPageDown: u32 = 0x4E;
pub const kHIDUsage_KeyboardRightArrow: u32 = 0x4F;
pub const kHIDUsage_KeyboardLeftArrow: u32 = 0x50;
pub const kHIDUsage_KeyboardDownArrow: u32 = 0x51;
pub const kHIDUsage_KeyboardUpArrow: u32 = 0x52;
pub const kHIDUsage_KeypadNumLock: u32 = 0x53;
pub const kHIDUsage_KeypadSlash: u32 = 0x54;
pub const kHIDUsage_KeypadAsterisk: u32 = 0x55;
pub const kHIDUsage_KeypadHyphen: u32 = 0x56;
pub const kHIDUsage_KeypadPlus: u32 = 0x57;
pub const kHIDUsage_KeypadEnter: u32 = 0x58;
pub const kHIDUsage_Keypad1: u32 = 0x59;
pub const kHIDUsage_Keypad2: u32 = 0x5A;
pub const kHIDUsage_Keypad3: u32 = 0x5B;
pub const kHIDUsage_Keypad4: u32 = 0x5C;
pub const kHIDUsage_Keypad5: u32 = 0x5D;
pub const kHIDUsage_Keypad6: u32 = 0x5E;
pub const kHIDUsage_Keypad7: u32 = 0x5F;
pub const kHIDUsage_Keypad8: u32 = 0x60;
pub const kHIDUsage_Keypad9: u32 = 0x61;
pub const kHIDUsage_Keypad0: u32 = 0x62;
pub const kHIDUsage_KeypadPeriod: u32 = 0x63;
pub const kHIDUsage_KeyboardNonUSBackslash: u32 = 0x64;
pub const kHIDUsage_KeyboardApplication: u32 = 0x65;
pub const kHIDUsage_KeyboardPower: u32 = 0x66;
pub const kHIDUsage_KeypadEqualSign: u32 = 0x67;
pub const kHIDUsage_KeyboardF13: u32 = 0x68;
pub const kHIDUsage_KeyboardF14: u32 = 0x69;
pub const kHIDUsage_KeyboardF15: u32 = 0x6A;
pub const kHIDUsage_KeyboardF16: u32 = 0x6B;
pub const kHIDUsage_KeyboardF17: u32 = 0x6C;
pub const kHIDUsage_KeyboardF18: u32 = 0x6D;
pub const kHIDUsage_KeyboardF19: u32 = 0x6E;
pub const kHIDUsage_KeyboardF20: u32 = 0x6F;
pub const kHIDUsage_KeyboardF21: u32 = 0x70;
pub const kHIDUsage_KeyboardF22: u32 = 0x71;
pub const kHIDUsage_KeyboardF23: u32 = 0x72;
pub const kHIDUsage_KeyboardF24: u32 = 0x73;
pub const kHIDUsage_KeyboardExecute: u32 = 0x74;
pub const kHIDUsage_KeyboardHelp: u32 = 0x75;
pub const kHIDUsage_KeyboardMenu: u32 = 0x76;
pub const kHIDUsage_KeyboardSelect: u32 = 0x77;
pub const kHIDUsage_KeyboardStop: u32 = 0x78;
pub const kHIDUsage_KeyboardAgain: u32 = 0x79;
pub const kHIDUsage_KeyboardUndo: u32 = 0x7A;
pub const kHIDUsage_KeyboardCut: u32 = 0x7B;
pub const kHIDUsage_KeyboardCopy: u32 = 0x7C;
pub const kHIDUsage_KeyboardPaste: u32 = 0x7D;
pub const kHIDUsage_KeyboardFind: u32 = 0x7E;
pub const kHIDUsage_KeyboardMute: u32 = 0x7F;
pub const kHIDUsage_KeyboardVolumeUp: u32 = 0x80;
pub const kHIDUsage_KeyboardVolumeDown: u32 = 0x81;
pub const kHIDUsage_KeyboardLockingCapsLock: u32 = 0x82;
pub const kHIDUsage_KeyboardLockingNumLock: u32 = 0x83;
pub const kHIDUsage_KeyboardLockingScrollLock: u32 = 0x84;
pub const kHIDUsage_KeypadComma: u32 = 0x85;
pub const kHIDUsage_KeypadEqualSignAS400: u32 = 0x86;
pub const kHIDUsage_KeyboardInternational1: u32 = 0x87;
pub const kHIDUsage_KeyboardInternational2: u32 = 0x88;
pub const kHIDUsage_KeyboardInternational3: u32 = 0x89;
pub const kHIDUsage_KeyboardInternational4: u32 = 0x8A;
pub const kHIDUsage_KeyboardInternational5: u32 = 0x8B;
pub const kHIDUsage_KeyboardInternational6: u32 = 0x8C;
pub const kHIDUsage_KeyboardInternational7: u32 = 0x8D;
pub const kHIDUsage_KeyboardInternational8: u32 = 0x8E;
pub const kHIDUsage_KeyboardInternational9: u32 = 0x8F;
pub const kHIDUsage_KeyboardLANG1: u32 = 0x90;
pub const kHIDUsage_KeyboardLANG2: u32 = 0x91;
pub const kHIDUsage_KeyboardLANG3: u32 = 0x92;
pub const kHIDUsage_KeyboardLANG4: u32 = 0x93;
pub const kHIDUsage_KeyboardLANG5: u32 = 0x94;
pub const kHIDUsage_KeyboardLANG6: u32 = 0x95;
pub const kHIDUsage_KeyboardLANG7: u32 = 0x96;
pub const kHIDUsage_KeyboardLANG8: u32 = 0x97;
pub const kHIDUsage_KeyboardLANG9: u32 = 0x98;
pub const kHIDUsage_KeyboardAlternateErase: u32 = 0x99;
pub const kHIDUsage_KeyboardSysReqOrAttention: u32 = 0x9A;
pub const kHIDUsage_KeyboardCancel: u32 = 0x9B;
pub const kHIDUsage_KeyboardClear: u32 = 0x9C;
pub const kHIDUsage_KeyboardPrior: u32 = 0x9D;
pub const kHIDUsage_KeyboardReturn: u32 = 0x9E;
pub const kHIDUsage_KeyboardSeparator: u32 = 0x9F;
pub const kHIDUsage_KeyboardOut: u32 = 0xA0;
pub const kHIDUsage_KeyboardOper: u32 = 0xA1;
pub const kHIDUsage_KeyboardClearOrAgain: u32 = 0xA2;
pub const kHIDUsage_KeyboardCrSelOrProps: u32 = 0xA3;
pub const kHIDUsage_KeyboardExSel: u32 = 0xA4;
// 0xA5-0xDF Reserved
pub const kHIDUsage_KeyboardLeftControl: u32 = 0xE0;
pub const kHIDUsage_KeyboardLeftShift: u32 = 0xE1;
pub const kHIDUsage_KeyboardLeftAlt: u32 = 0xE2;
pub const kHIDUsage_KeyboardLeftGUI: u32 = 0xE3;
pub const kHIDUsage_KeyboardRightControl: u32 = 0xE4;
pub const kHIDUsage_KeyboardRightShift: u32 = 0xE5;
pub const kHIDUsage_KeyboardRightAlt: u32 = 0xE6;
pub const kHIDUsage_KeyboardRightGUI: u32 = 0xE7;
// 0xE8-0xFFFF Reserved
pub const kHIDUsage_Keyboard_Reserved: u32 = 0xFFFF;

// LEDs Page (0x08)
pub const kHIDUsage_LED_NumLock: u32 = 0x01;
pub const kHIDUsage_LED_CapsLock: u32 = 0x02;
pub const kHIDUsage_LED_ScrollLock: u32 = 0x03;
pub const kHIDUsage_LED_Compose: u32 = 0x04;
pub const kHIDUsage_LED_Kana: u32 = 0x05;
pub const kHIDUsage_LED_Power: u32 = 0x06;
pub const kHIDUsage_LED_Shift: u32 = 0x07;
pub const kHIDUsage_LED_DoNotDisturb: u32 = 0x08;
pub const kHIDUsage_LED_Mute: u32 = 0x09;
pub const kHIDUsage_LED_ToneEnable: u32 = 0x0A;
pub const kHIDUsage_LED_HighCutFilter: u32 = 0x0B;
pub const kHIDUsage_LED_LowCutFilter: u32 = 0x0C;
pub const kHIDUsage_LED_EqualizerEnable: u32 = 0x0D;
pub const kHIDUsage_LED_SoundFieldOn: u32 = 0x0E;
pub const kHIDUsage_LED_SurroundOn: u32 = 0x0F;
pub const kHIDUsage_LED_Repeat: u32 = 0x10;
pub const kHIDUsage_LED_Stereo: u32 = 0x11;
pub const kHIDUsage_LED_SamplingRateDetect: u32 = 0x12;
pub const kHIDUsage_LED_Spinning: u32 = 0x13;
pub const kHIDUsage_LED_CAV: u32 = 0x14;
pub const kHIDUsage_LED_CLV: u32 = 0x15;
pub const kHIDUsage_LED_RecordingFormatDetect: u32 = 0x16;
pub const kHIDUsage_LED_OffHook: u32 = 0x17;
pub const kHIDUsage_LED_Ring: u32 = 0x18;
pub const kHIDUsage_LED_MessageWaiting: u32 = 0x19;
pub const kHIDUsage_LED_DataMode: u32 = 0x1A;
pub const kHIDUsage_LED_BatteryOperation: u32 = 0x1B;
pub const kHIDUsage_LED_BatteryOK: u32 = 0x1C;
pub const kHIDUsage_LED_BatteryLow: u32 = 0x1D;
pub const kHIDUsage_LED_Speaker: u32 = 0x1E;
pub const kHIDUsage_LED_HeadSet: u32 = 0x1F;
pub const kHIDUsage_LED_Hold: u32 = 0x20;
pub const kHIDUsage_LED_Microphone: u32 = 0x21;
pub const kHIDUsage_LED_Coverage: u32 = 0x22;
pub const kHIDUsage_LED_NightMode: u32 = 0x23;
pub const kHIDUsage_LED_SendCalls: u32 = 0x24;
pub const kHIDUsage_LED_CallPickup: u32 = 0x25;
pub const kHIDUsage_LED_Conference: u32 = 0x26;
pub const kHIDUsage_LED_StandBy: u32 = 0x27;
pub const kHIDUsage_LED_CameraOn: u32 = 0x28;
pub const kHIDUsage_LED_CameraOff: u32 = 0x29;
pub const kHIDUsage_LED_OnLine: u32 = 0x2A;
pub const kHIDUsage_LED_OffLine: u32 = 0x2B;
pub const kHIDUsage_LED_Busy: u32 = 0x2C;
pub const kHIDUsage_LED_Ready: u32 = 0x2D;
pub const kHIDUsage_LED_PaperOut: u32 = 0x2E;
pub const kHIDUsage_LED_PaperJam: u32 = 0x2F;
pub const kHIDUsage_LED_Remote: u32 = 0x30;
pub const kHIDUsage_LED_Forward: u32 = 0x31;
pub const kHIDUsage_LED_Reverse: u32 = 0x32;
pub const kHIDUsage_LED_Stop: u32 = 0x33;
pub const kHIDUsage_LED_Rewind: u32 = 0x34;
pub const kHIDUsage_LED_FastForward: u32 = 0x35;
pub const kHIDUsage_LED_Play: u32 = 0x36;
pub const kHIDUsage_LED_Pause: u32 = 0x37;
pub const kHIDUsage_LED_Record: u32 = 0x38;
pub const kHIDUsage_LED_Error: u32 = 0x39;
pub const kHIDUsage_LED_Usage: u32 = 0x3A;
pub const kHIDUsage_LED_UsageInUseIndicator: u32 = 0x3B;
pub const kHIDUsage_LED_UsageMultiModeIndicator: u32 = 0x3C;
pub const kHIDUsage_LED_IndicatorOn: u32 = 0x3D;
pub const kHIDUsage_LED_IndicatorFlash: u32 = 0x3E;
pub const kHIDUsage_LED_IndicatorSlowBlink: u32 = 0x3F;
pub const kHIDUsage_LED_IndicatorFastBlink: u32 = 0x40;
pub const kHIDUsage_LED_IndicatorOff: u32 = 0x41;
pub const kHIDUsage_LED_FlashOnTime: u32 = 0x42;
pub const kHIDUsage_LED_SlowBlinkOnTime: u32 = 0x43;
pub const kHIDUsage_LED_SlowBlinkOffTime: u32 = 0x44;
pub const kHIDUsage_LED_FastBlinkOnTime: u32 = 0x45;
pub const kHIDUsage_LED_FastBlinkOffTime: u32 = 0x46;
pub const kHIDUsage_LED_UsageIndicatorColor: u32 = 0x47;
pub const kHIDUsage_LED_IndicatorRed: u32 = 0x48;
pub const kHIDUsage_LED_IndicatorGreen: u32 = 0x49;
pub const kHIDUsage_LED_IndicatorAmber: u32 = 0x4A;
pub const kHIDUsage_LED_GenericIndicator: u32 = 0x4B;
pub const kHIDUsage_LED_SystemSuspend: u32 = 0x4C;
pub const kHIDUsage_LED_ExternalPowerConnected: u32 = 0x4D;
// 0x4E - 0xFFFF Reserved
pub const kHIDUsage_LED_Reserved: u32 = 0xFFFF;

// Button Page (0x09)
pub const kHIDUsage_Button_1: u32 = 0x01;
pub const kHIDUsage_Button_2: u32 = 0x02;
pub const kHIDUsage_Button_3: u32 = 0x03;
pub const kHIDUsage_Button_4: u32 = 0x04;
// ...
pub const kHIDUsage_Button_65535: u32 = 0xFFFF;
// Ordinal Page (0x0A)
// 0x00 Reserved
pub const kHIDUsage_Ord_Instance1: u32 = 0x01;
pub const kHIDUsage_Ord_Instance2: u32 = 0x02;
pub const kHIDUsage_Ord_Instance3: u32 = 0x03;
pub const kHIDUsage_Ord_Instance4: u32 = 0x04;
pub const kHIDUsage_Ord_Instance65535: u32 = 0xFFFF;

// Telephony Page (0x0B)
pub const kHIDUsage_Tfon_Phone: u32 = 0x01;
pub const kHIDUsage_Tfon_AnsweringMachine: u32 = 0x02;
pub const kHIDUsage_Tfon_MessageControls: u32 = 0x03;
pub const kHIDUsage_Tfon_Handset: u32 = 0x04;
pub const kHIDUsage_Tfon_Headset: u32 = 0x05;
pub const kHIDUsage_Tfon_TelephonyKeyPad: u32 = 0x06;
pub const kHIDUsage_Tfon_ProgrammableButton: u32 = 0x07;
// 0x08 - 0x1F Reserved
pub const kHIDUsage_Tfon_HookSwitch: u32 = 0x20;
pub const kHIDUsage_Tfon_Flash: u32 = 0x21;
pub const kHIDUsage_Tfon_Feature: u32 = 0x22;
pub const kHIDUsage_Tfon_Hold: u32 = 0x23;
pub const kHIDUsage_Tfon_Redial: u32 = 0x24;
pub const kHIDUsage_Tfon_Transfer: u32 = 0x25;
pub const kHIDUsage_Tfon_Drop: u32 = 0x26;
pub const kHIDUsage_Tfon_Park: u32 = 0x27;
pub const kHIDUsage_Tfon_ForwardCalls: u32 = 0x28;
pub const kHIDUsage_Tfon_AlternateFunction: u32 = 0x29;
pub const kHIDUsage_Tfon_Line: u32 = 0x2A;
pub const kHIDUsage_Tfon_SpeakerPhone: u32 = 0x2B;
pub const kHIDUsage_Tfon_Conference: u32 = 0x2C;
pub const kHIDUsage_Tfon_RingEnable: u32 = 0x2D;
pub const kHIDUsage_Tfon_Ring: u32 = 0x2E;
pub const kHIDUsage_Tfon_PhoneMute: u32 = 0x2F;
pub const kHIDUsage_Tfon_CallerID: u32 = 0x30;
// 0x31 - 0x4F Reserved
pub const kHIDUsage_Tfon_SpeedDial: u32 = 0x50;
pub const kHIDUsage_Tfon_StoreNumber: u32 = 0x51;
pub const kHIDUsage_Tfon_RecallNumber: u32 = 0x52;
pub const kHIDUsage_Tfon_PhoneDirectory: u32 = 0x53;
// 0x54 - 0x6F Reserved
pub const kHIDUsage_Tfon_VoiceMail: u32 = 0x70;
pub const kHIDUsage_Tfon_ScreenCalls: u32 = 0x71;
pub const kHIDUsage_Tfon_DoNotDisturb: u32 = 0x72;
pub const kHIDUsage_Tfon_Message: u32 = 0x73;
pub const kHIDUsage_Tfon_AnswerOnOrOff: u32 = 0x74;
// 0x75 - 0x8F Reserved
pub const kHIDUsage_Tfon_InsideDialTone: u32 = 0x90;
pub const kHIDUsage_Tfon_OutsideDialTone: u32 = 0x91;
pub const kHIDUsage_Tfon_InsideRingTone: u32 = 0x92;
pub const kHIDUsage_Tfon_OutsideRingTone: u32 = 0x93;
pub const kHIDUsage_Tfon_PriorityRingTone: u32 = 0x94;
pub const kHIDUsage_Tfon_InsideRingback: u32 = 0x95;
pub const kHIDUsage_Tfon_PriorityRingback: u32 = 0x96;
pub const kHIDUsage_Tfon_LineBusyTone: u32 = 0x97;
pub const kHIDUsage_Tfon_ReorderTone: u32 = 0x98;
pub const kHIDUsage_Tfon_CallWaitingTone: u32 = 0x99;
pub const kHIDUsage_Tfon_ConfirmationTone1: u32 = 0x9A;
pub const kHIDUsage_Tfon_ConfirmationTone2: u32 = 0x9B;
pub const kHIDUsage_Tfon_TonesOff: u32 = 0x9C;
pub const kHIDUsage_Tfon_OutsideRingback: u32 = 0x9D;
// 0x9E - 0xAF Reserved
pub const kHIDUsage_Tfon_PhoneKey0: u32 = 0xB0;
pub const kHIDUsage_Tfon_PhoneKey1: u32 = 0xB1;
pub const kHIDUsage_Tfon_PhoneKey2: u32 = 0xB2;
pub const kHIDUsage_Tfon_PhoneKey3: u32 = 0xB3;
pub const kHIDUsage_Tfon_PhoneKey4: u32 = 0xB4;
pub const kHIDUsage_Tfon_PhoneKey5: u32 = 0xB5;
pub const kHIDUsage_Tfon_PhoneKey6: u32 = 0xB6;
pub const kHIDUsage_Tfon_PhoneKey7: u32 = 0xB7;
pub const kHIDUsage_Tfon_PhoneKey8: u32 = 0xB8;
pub const kHIDUsage_Tfon_PhoneKey9: u32 = 0xB9;
pub const kHIDUsage_Tfon_PhoneKeyStar: u32 = 0xBA;
pub const kHIDUsage_Tfon_PhoneKeyPound: u32 = 0xBB;
pub const kHIDUsage_Tfon_PhoneKeyA: u32 = 0xBC;
pub const kHIDUsage_Tfon_PhoneKeyB: u32 = 0xBD;
pub const kHIDUsage_Tfon_PhoneKeyC: u32 = 0xBE;
pub const kHIDUsage_Tfon_PhoneKeyD: u32 = 0xBF;
// 0xC0 - 0xFFFF Reserved
pub const kHIDUsage_TFon_Reserved: u32 = 0xFFFF;

// Consumer Page (0x0C)
pub const kHIDUsage_Csmr_ConsumerControl: u32 = 0x01;
pub const kHIDUsage_Csmr_NumericKeyPad: u32 = 0x02;
pub const kHIDUsage_Csmr_ProgrammableButtons: u32 = 0x03;
pub const kHIDUsage_Csmr_Microphone: u32 = 0x04;
pub const kHIDUsage_Csmr_Headphone: u32 = 0x05;
pub const kHIDUsage_Csmr_GraphicEqualizer: u32 = 0x06;
// 0x07 - 0x1F Reserved
pub const kHIDUsage_Csmr_Plus10: u32 = 0x20;
pub const kHIDUsage_Csmr_Plus100: u32 = 0x21;
pub const kHIDUsage_Csmr_AMOrPM: u32 = 0x22;
// 0x23 - 0x3F Reserved
pub const kHIDUsage_Csmr_Power: u32 = 0x30;
pub const kHIDUsage_Csmr_Reset: u32 = 0x31;
pub const kHIDUsage_Csmr_Sleep: u32 = 0x32;
pub const kHIDUsage_Csmr_SleepAfter: u32 = 0x33;
pub const kHIDUsage_Csmr_SleepMode: u32 = 0x34;
pub const kHIDUsage_Csmr_Illumination: u32 = 0x35;
pub const kHIDUsage_Csmr_FunctionButtons: u32 = 0x36;
// 0x37 - 0x3F Reserved
pub const kHIDUsage_Csmr_Menu: u32 = 0x40;
pub const kHIDUsage_Csmr_MenuPick: u32 = 0x41;
pub const kHIDUsage_Csmr_MenuUp: u32 = 0x42;
pub const kHIDUsage_Csmr_MenuDown: u32 = 0x43;
pub const kHIDUsage_Csmr_MenuLeft: u32 = 0x44;
pub const kHIDUsage_Csmr_MenuRight: u32 = 0x45;
pub const kHIDUsage_Csmr_MenuEscape: u32 = 0x46;
pub const kHIDUsage_Csmr_MenuValueIncrease: u32 = 0x47;
pub const kHIDUsage_Csmr_MenuValueDecrease: u32 = 0x48;
// 0x49 - 0x5F Reserved
pub const kHIDUsage_Csmr_DataOnScreen: u32 = 0x60;
pub const kHIDUsage_Csmr_ClosedCaption: u32 = 0x61;
pub const kHIDUsage_Csmr_ClosedCaptionSelect: u32 = 0x62;
pub const kHIDUsage_Csmr_VCROrTV: u32 = 0x63;
pub const kHIDUsage_Csmr_BroadcastMode: u32 = 0x64;
pub const kHIDUsage_Csmr_Snapshot: u32 = 0x65;
pub const kHIDUsage_Csmr_Still: u32 = 0x66;
pub const kHIDUsage_Csmr_PictureInPictureToggle: u32 = 0x67;
pub const kHIDUsage_Csmr_PictureInPictureSwap: u32 = 0x68;
pub const kHIDUsage_Csmr_RedMenuButton: u32 = 0x69;
pub const kHIDUsage_Csmr_GreenMenuButton: u32 = 0x6A;
pub const kHIDUsage_Csmr_BlueMenuButton: u32 = 0x6B;
pub const kHIDUsage_Csmr_YellowMenuButton: u32 = 0x6C;
pub const kHIDUsage_Csmr_Aspect: u32 = 0x6D;
pub const kHIDUsage_Csmr_3DModeSelect: u32 = 0x6E;
pub const kHIDUsage_Csmr_DisplayBrightnessIncrement: u32 = 0x6F;
pub const kHIDUsage_Csmr_DisplayBrightnessDecrement: u32 = 0x70;
pub const kHIDUsage_Csmr_DisplayBrightness: u32 = 0x71;
pub const kHIDUsage_Csmr_DisplayBacklightToggle: u32 = 0x72;
pub const kHIDUsage_Csmr_DisplayBrightnessMinimum: u32 = 0x73;
pub const kHIDUsage_Csmr_DisplayBrightnessMaximum: u32 = 0x74;
pub const kHIDUsage_Csmr_DisplayBrightnessSetAutoBrightness: u32 = 0x75;
// 0x76 - 0x7F Reserved
pub const kHIDUsage_Csmr_Selection: u32 = 0x80;
pub const kHIDUsage_Csmr_Assign: u32 = 0x81;
pub const kHIDUsage_Csmr_ModeStep: u32 = 0x82;
pub const kHIDUsage_Csmr_RecallLast: u32 = 0x83;
pub const kHIDUsage_Csmr_EnterChannel: u32 = 0x84;
pub const kHIDUsage_Csmr_OrderMovie: u32 = 0x85;
pub const kHIDUsage_Csmr_Channel: u32 = 0x86;
pub const kHIDUsage_Csmr_MediaSelection: u32 = 0x87;
pub const kHIDUsage_Csmr_MediaSelectComputer: u32 = 0x88;
pub const kHIDUsage_Csmr_MediaSelectTV: u32 = 0x89;
pub const kHIDUsage_Csmr_MediaSelectWWW: u32 = 0x8A;
pub const kHIDUsage_Csmr_MediaSelectDVD: u32 = 0x8B;
pub const kHIDUsage_Csmr_MediaSelectTelephone: u32 = 0x8C;
pub const kHIDUsage_Csmr_MediaSelectProgramGuide: u32 = 0x8D;
pub const kHIDUsage_Csmr_MediaSelectVideoPhone: u32 = 0x8E;
pub const kHIDUsage_Csmr_MediaSelectGames: u32 = 0x8F;
pub const kHIDUsage_Csmr_MediaSelectMessages: u32 = 0x90;
pub const kHIDUsage_Csmr_MediaSelectCD: u32 = 0x91;
pub const kHIDUsage_Csmr_MediaSelectVCR: u32 = 0x92;
pub const kHIDUsage_Csmr_MediaSelectTuner: u32 = 0x93;
pub const kHIDUsage_Csmr_Quit: u32 = 0x94;
pub const kHIDUsage_Csmr_Help: u32 = 0x95;
pub const kHIDUsage_Csmr_MediaSelectTape: u32 = 0x96;
pub const kHIDUsage_Csmr_MediaSelectCable: u32 = 0x97;
pub const kHIDUsage_Csmr_MediaSelectSatellite: u32 = 0x98;
pub const kHIDUsage_Csmr_MediaSelectSecurity: u32 = 0x99;
pub const kHIDUsage_Csmr_MediaSelectHome: u32 = 0x9A;
pub const kHIDUsage_Csmr_MediaSelectCall: u32 = 0x9B;
pub const kHIDUsage_Csmr_ChannelIncrement: u32 = 0x9C;
pub const kHIDUsage_Csmr_ChannelDecrement: u32 = 0x9D;
pub const kHIDUsage_Csmr_Media: u32 = 0x9E;
// 0x9F Reserved
pub const kHIDUsage_Csmr_VCRPlus: u32 = 0xA0;
pub const kHIDUsage_Csmr_Once: u32 = 0xA1;
pub const kHIDUsage_Csmr_Daily: u32 = 0xA2;
pub const kHIDUsage_Csmr_Weekly: u32 = 0xA3;
pub const kHIDUsage_Csmr_Monthly: u32 = 0xA4;
// 0xA5 - 0xAF Reserved
pub const kHIDUsage_Csmr_Play: u32 = 0xB0;
pub const kHIDUsage_Csmr_Pause: u32 = 0xB1;
pub const kHIDUsage_Csmr_Record: u32 = 0xB2;
pub const kHIDUsage_Csmr_FastForward: u32 = 0xB3;
pub const kHIDUsage_Csmr_Rewind: u32 = 0xB4;
pub const kHIDUsage_Csmr_ScanNextTrack: u32 = 0xB5;
pub const kHIDUsage_Csmr_ScanPreviousTrack: u32 = 0xB6;
pub const kHIDUsage_Csmr_Stop: u32 = 0xB7;
pub const kHIDUsage_Csmr_Eject: u32 = 0xB8;
pub const kHIDUsage_Csmr_RandomPlay: u32 = 0xB9;
pub const kHIDUsage_Csmr_SelectDisc: u32 = 0xBA;
pub const kHIDUsage_Csmr_EnterDisc: u32 = 0xBB;
pub const kHIDUsage_Csmr_Repeat: u32 = 0xBC;
pub const kHIDUsage_Csmr_Tracking: u32 = 0xBD;
pub const kHIDUsage_Csmr_TrackNormal: u32 = 0xBE;
pub const kHIDUsage_Csmr_SlowTracking: u32 = 0xBF;
pub const kHIDUsage_Csmr_FrameForward: u32 = 0xC0;
pub const kHIDUsage_Csmr_FrameBack: u32 = 0xC1;
pub const kHIDUsage_Csmr_Mark: u32 = 0xC2;
pub const kHIDUsage_Csmr_ClearMark: u32 = 0xC3;
pub const kHIDUsage_Csmr_RepeatFromMark: u32 = 0xC4;
pub const kHIDUsage_Csmr_ReturnToMark: u32 = 0xC5;
pub const kHIDUsage_Csmr_SearchMarkForward: u32 = 0xC6;
pub const kHIDUsage_Csmr_SearchMarkBackwards: u32 = 0xC7;
pub const kHIDUsage_Csmr_CounterReset: u32 = 0xC8;
pub const kHIDUsage_Csmr_ShowCounter: u32 = 0xC9;
pub const kHIDUsage_Csmr_TrackingIncrement: u32 = 0xCA;
pub const kHIDUsage_Csmr_TrackingDecrement: u32 = 0xCB;
pub const kHIDUsage_Csmr_StopOrEject: u32 = 0xCC;
pub const kHIDUsage_Csmr_PlayOrPause: u32 = 0xCD;
pub const kHIDUsage_Csmr_PlayOrSkip: u32 = 0xCE;
pub const kHIDUsage_Csmr_VoiceCommand: u32 = 0xCF;
// 0xCF - 0xDF Reserved
pub const kHIDUsage_Csmr_Volume: u32 = 0xE0;
pub const kHIDUsage_Csmr_Balance: u32 = 0xE1;
pub const kHIDUsage_Csmr_Mute: u32 = 0xE2;
pub const kHIDUsage_Csmr_Bass: u32 = 0xE3;
pub const kHIDUsage_Csmr_Treble: u32 = 0xE4;
pub const kHIDUsage_Csmr_BassBoost: u32 = 0xE5;
pub const kHIDUsage_Csmr_SurroundMode: u32 = 0xE6;
pub const kHIDUsage_Csmr_Loudness: u32 = 0xE7;
pub const kHIDUsage_Csmr_MPX: u32 = 0xE8;
pub const kHIDUsage_Csmr_VolumeIncrement: u32 = 0xE9;
pub const kHIDUsage_Csmr_VolumeDecrement: u32 = 0xEA;
// 0xEB - 0xEF Reserved
pub const kHIDUsage_Csmr_Speed: u32 = 0xF0;
pub const kHIDUsage_Csmr_PlaybackSpeed: u32 = 0xF1;
pub const kHIDUsage_Csmr_StandardPlay: u32 = 0xF2;
pub const kHIDUsage_Csmr_LongPlay: u32 = 0xF3;
pub const kHIDUsage_Csmr_ExtendedPlay: u32 = 0xF4;
pub const kHIDUsage_Csmr_Slow: u32 = 0xF5;
// 0xF6 - 0xFF Reserved
pub const kHIDUsage_Csmr_FanEnable: u32 = 0x100;
pub const kHIDUsage_Csmr_FanSpeed: u32 = 0x101;
pub const kHIDUsage_Csmr_LightEnable: u32 = 0x102;
pub const kHIDUsage_Csmr_LightIlluminationLevel: u32 = 0x103;
pub const kHIDUsage_Csmr_ClimateControlEnable: u32 = 0x104;
pub const kHIDUsage_Csmr_RoomTemperature: u32 = 0x105;
pub const kHIDUsage_Csmr_SecurityEnable: u32 = 0x106;
pub const kHIDUsage_Csmr_FireAlarm: u32 = 0x107;
pub const kHIDUsage_Csmr_PoliceAlarm: u32 = 0x108;
pub const kHIDUsage_Csmr_Proximity: u32 = 0x109;
pub const kHIDUsage_Csmr_Motion: u32 = 0x10A;
pub const kHIDUsage_Csmr_DuressAlarm: u32 = 0x10B;
pub const kHIDUsage_Csmr_HoldupAlarm: u32 = 0x10C;
pub const kHIDUsage_Csmr_MedicalAlarm: u32 = 0x10D;
// 0x10E - 0x14F Reserved
pub const kHIDUsage_Csmr_BalanceRight: u32 = 0x150;
pub const kHIDUsage_Csmr_BalanceLeft: u32 = 0x151;
pub const kHIDUsage_Csmr_BassIncrement: u32 = 0x152;
pub const kHIDUsage_Csmr_BassDecrement: u32 = 0x153;
pub const kHIDUsage_Csmr_TrebleIncrement: u32 = 0x154;
pub const kHIDUsage_Csmr_TrebleDecrement: u32 = 0x155;
// 0x156 - 0x15F Reserved
pub const kHIDUsage_Csmr_SpeakerSystem: u32 = 0x160;
pub const kHIDUsage_Csmr_ChannelLeft: u32 = 0x161;
pub const kHIDUsage_Csmr_ChannelRight: u32 = 0x162;
pub const kHIDUsage_Csmr_ChannelCenter: u32 = 0x163;
pub const kHIDUsage_Csmr_ChannelFront: u32 = 0x164;
pub const kHIDUsage_Csmr_ChannelCenterFront: u32 = 0x165;
pub const kHIDUsage_Csmr_ChannelSide: u32 = 0x166;
pub const kHIDUsage_Csmr_ChannelSurround: u32 = 0x167;
pub const kHIDUsage_Csmr_ChannelLowFrequencyEnhancement: u32 = 0x168;
pub const kHIDUsage_Csmr_ChannelTop: u32 = 0x169;
pub const kHIDUsage_Csmr_ChannelUnknown: u32 = 0x16A;
// 0x16B - 0x16F Reserved
pub const kHIDUsage_Csmr_SubChannel: u32 = 0x170;
pub const kHIDUsage_Csmr_SubChannelIncrement: u32 = 0x171;
pub const kHIDUsage_Csmr_SubChannelDecrement: u32 = 0x172;
pub const kHIDUsage_Csmr_AlternateAudioIncrement: u32 = 0x173;
pub const kHIDUsage_Csmr_AlternateAudioDecrement: u32 = 0x174;
// 0x175 - 0x17F Reserved
pub const kHIDUsage_Csmr_ApplicationLaunchButtons: u32 = 0x180;
pub const kHIDUsage_Csmr_ALLaunchButtonConfigurationTool: u32 = 0x181;
pub const kHIDUsage_Csmr_ALProgrammableButtonConfiguration: u32 = 0x182;
pub const kHIDUsage_Csmr_ALConsumerControlConfiguration: u32 = 0x183;
pub const kHIDUsage_Csmr_ALWordProcessor: u32 = 0x184;
pub const kHIDUsage_Csmr_ALTextEditor: u32 = 0x185;
pub const kHIDUsage_Csmr_ALSpreadsheet: u32 = 0x186;
pub const kHIDUsage_Csmr_ALGraphicsEditor: u32 = 0x187;
pub const kHIDUsage_Csmr_ALPresentationApp: u32 = 0x188;
pub const kHIDUsage_Csmr_ALDatabaseApp: u32 = 0x189;
pub const kHIDUsage_Csmr_ALEmailReader: u32 = 0x18A;
pub const kHIDUsage_Csmr_ALNewsreader: u32 = 0x18B;
pub const kHIDUsage_Csmr_ALVoicemail: u32 = 0x18C;
pub const kHIDUsage_Csmr_ALContactsOrAddressBook: u32 = 0x18D;
pub const kHIDUsage_Csmr_ALCalendarOrSchedule: u32 = 0x18E;
pub const kHIDUsage_Csmr_ALTaskOrProjectManager: u32 = 0x18F;
pub const kHIDUsage_Csmr_ALLogOrJournalOrTimecard: u32 = 0x190;
pub const kHIDUsage_Csmr_ALCheckbookOrFinance: u32 = 0x191;
pub const kHIDUsage_Csmr_ALCalculator: u32 = 0x192;
pub const kHIDUsage_Csmr_ALAOrVCaptureOrPlayback: u32 = 0x193;
pub const kHIDUsage_Csmr_ALLocalMachineBrowser: u32 = 0x194;
pub const kHIDUsage_Csmr_ALLANOrWANBrowser: u32 = 0x195;
pub const kHIDUsage_Csmr_ALInternetBrowser: u32 = 0x196;
pub const kHIDUsage_Csmr_ALRemoteNetworkingOrISPConnect: u32 = 0x197;
pub const kHIDUsage_Csmr_ALNetworkConference: u32 = 0x198;
pub const kHIDUsage_Csmr_ALNetworkChat: u32 = 0x199;
pub const kHIDUsage_Csmr_ALTelephonyOrDialer: u32 = 0x19A;
pub const kHIDUsage_Csmr_ALLogon: u32 = 0x19B;
pub const kHIDUsage_Csmr_ALLogoff: u32 = 0x19C;
pub const kHIDUsage_Csmr_ALLogonOrLogoff: u32 = 0x19D;
pub const kHIDUsage_Csmr_ALTerminalLockOrScreensaver: u32 = 0x19E;
pub const kHIDUsage_Csmr_ALControlPanel: u32 = 0x19F;
pub const kHIDUsage_Csmr_ALCommandLineProcessorOrRun: u32 = 0x1A0;
pub const kHIDUsage_Csmr_ALProcessOrTaskManager: u32 = 0x1A1;
pub const kHIDUsage_Csmr_AL: u32 = 0x1A2;
pub const kHIDUsage_Csmr_ALNextTaskOrApplication: u32 = 0x1A3;
pub const kHIDUsage_Csmr_ALPreviousTaskOrApplication: u32 = 0x1A4;
pub const kHIDUsage_Csmr_ALPreemptiveHaltTaskOrApplication: u32 = 0x1A5;
pub const kHIDUsage_Csmr_ALIntegratedHelpCenter: u32 = 0x1A6;
pub const kHIDUsage_Csmr_ALDocuments: u32 = 0x1A7;
pub const kHIDUsage_Csmr_ALThesaurus: u32 = 0x1A8;
pub const kHIDUsage_Csmr_ALDictionary: u32 = 0x1A9;
pub const kHIDUsage_Csmr_ALDesktop: u32 = 0x1AA;
pub const kHIDUsage_Csmr_ALSpellCheck: u32 = 0x1AB;
pub const kHIDUsage_Csmr_ALGrammerCheck: u32 = 0x1AC;
pub const kHIDUsage_Csmr_ALWirelessStatus: u32 = 0x1AD;
pub const kHIDUsage_Csmr_ALKeyboardLayout: u32 = 0x1AE;
pub const kHIDUsage_Csmr_ALVirusProtection: u32 = 0x1AF;
pub const kHIDUsage_Csmr_ALEncryption: u32 = 0x1B0;
pub const kHIDUsage_Csmr_ALScreenSaver: u32 = 0x1B1;
pub const kHIDUsage_Csmr_ALAlarms: u32 = 0x1B2;
pub const kHIDUsage_Csmr_ALClock: u32 = 0x1B3;
pub const kHIDUsage_Csmr_ALFileBrowser: u32 = 0x1B4;
pub const kHIDUsage_Csmr_ALPowerStatus: u32 = 0x1B5;
pub const kHIDUsage_Csmr_ALImageBrowser: u32 = 0x1B6;
pub const kHIDUsage_Csmr_ALAudioBrowser: u32 = 0x1B7;
pub const kHIDUsage_Csmr_ALMovieBrowser: u32 = 0x1B8;
pub const kHIDUsage_Csmr_ALDigitalRightsManager: u32 = 0x1B9;
pub const kHIDUsage_Csmr_ALDigitalWallet: u32 = 0x1BA;
// 0x1BB Reserved
pub const kHIDUsage_Csmr_ALInstantMessaging: u32 = 0x1BC;
pub const kHIDUsage_Csmr_ALOEMFeatureBrowser: u32 = 0x1BD;
pub const kHIDUsage_Csmr_ALOEMHelp: u32 = 0x1BE;
pub const kHIDUsage_Csmr_ALOnlineCommunity: u32 = 0x1BF;
pub const kHIDUsage_Csmr_ALEntertainmentContentBrowser: u32 = 0x1C0;
pub const kHIDUsage_Csmr_ALOnlineShoppingBrowswer: u32 = 0x1C1;
pub const kHIDUsage_Csmr_ALSmartCardInformationOrHelp: u32 = 0x1C2;
pub const kHIDUsage_Csmr_ALMarketMonitorOrFinanceBrowser: u32 = 0x1C3;
pub const kHIDUsage_Csmr_ALCustomizedCorporateNewsBrowser: u32 = 0x1C4;
pub const kHIDUsage_Csmr_ALOnlineActivityBrowswer: u32 = 0x1C5;
pub const kHIDUsage_Csmr_ALResearchOrSearchBrowswer: u32 = 0x1C6;
pub const kHIDUsage_Csmr_ALAudioPlayer: u32 = 0x1C7;
// 0x1C8 - 0x1FF Reserved
pub const kHIDUsage_Csmr_GenericGUIApplicationControls: u32 = 0x200;
pub const kHIDUsage_Csmr_ACNew: u32 = 0x201;
pub const kHIDUsage_Csmr_ACOpen: u32 = 0x202;
pub const kHIDUsage_Csmr_ACClose: u32 = 0x203;
pub const kHIDUsage_Csmr_ACExit: u32 = 0x204;
pub const kHIDUsage_Csmr_ACMaximize: u32 = 0x205;
pub const kHIDUsage_Csmr_ACMinimize: u32 = 0x206;
pub const kHIDUsage_Csmr_ACSave: u32 = 0x207;
pub const kHIDUsage_Csmr_ACPrint: u32 = 0x208;
pub const kHIDUsage_Csmr_ACProperties: u32 = 0x209;
pub const kHIDUsage_Csmr_ACUndo: u32 = 0x21A;
pub const kHIDUsage_Csmr_ACCopy: u32 = 0x21B;
pub const kHIDUsage_Csmr_ACCut: u32 = 0x21C;
pub const kHIDUsage_Csmr_ACPaste: u32 = 0x21D;
pub const kHIDUsage_Csmr_AC: u32 = 0x21E;
pub const kHIDUsage_Csmr_ACFind: u32 = 0x21F;
pub const kHIDUsage_Csmr_ACFindandReplace: u32 = 0x220;
pub const kHIDUsage_Csmr_ACSearch: u32 = 0x221;
pub const kHIDUsage_Csmr_ACGoTo: u32 = 0x222;
pub const kHIDUsage_Csmr_ACHome: u32 = 0x223;
pub const kHIDUsage_Csmr_ACBack: u32 = 0x224;
pub const kHIDUsage_Csmr_ACForward: u32 = 0x225;
pub const kHIDUsage_Csmr_ACStop: u32 = 0x226;
pub const kHIDUsage_Csmr_ACRefresh: u32 = 0x227;
pub const kHIDUsage_Csmr_ACPreviousLink: u32 = 0x228;
pub const kHIDUsage_Csmr_ACNextLink: u32 = 0x229;
pub const kHIDUsage_Csmr_ACBookmarks: u32 = 0x22A;
pub const kHIDUsage_Csmr_ACHistory: u32 = 0x22B;
pub const kHIDUsage_Csmr_ACSubscriptions: u32 = 0x22C;
pub const kHIDUsage_Csmr_ACZoomIn: u32 = 0x22D;
pub const kHIDUsage_Csmr_ACZoomOut: u32 = 0x22E;
pub const kHIDUsage_Csmr_ACZoom: u32 = 0x22F;
pub const kHIDUsage_Csmr_ACFullScreenView: u32 = 0x230;
pub const kHIDUsage_Csmr_ACNormalView: u32 = 0x231;
pub const kHIDUsage_Csmr_ACViewToggle: u32 = 0x232;
pub const kHIDUsage_Csmr_ACScrollUp: u32 = 0x233;
pub const kHIDUsage_Csmr_ACScrollDown: u32 = 0x234;
pub const kHIDUsage_Csmr_ACScroll: u32 = 0x235;
pub const kHIDUsage_Csmr_ACPanLeft: u32 = 0x236;
pub const kHIDUsage_Csmr_ACPanRight: u32 = 0x237;
pub const kHIDUsage_Csmr_ACPan: u32 = 0x238;
pub const kHIDUsage_Csmr_ACNewWindow: u32 = 0x239;
pub const kHIDUsage_Csmr_ACTileHorizontally: u32 = 0x23A;
pub const kHIDUsage_Csmr_ACTileVertically: u32 = 0x23B;
pub const kHIDUsage_Csmr_ACFormat: u32 = 0x23C;
pub const kHIDUsage_Csmr_ACEdit: u32 = 0x23D;
pub const kHIDUsage_Csmr_ACBold: u32 = 0x23E;
pub const kHIDUsage_Csmr_ACItalics: u32 = 0x23F;
pub const kHIDUsage_Csmr_ACUnderline: u32 = 0x240;
pub const kHIDUsage_Csmr_ACStrikethrough: u32 = 0x241;
pub const kHIDUsage_Csmr_ACSubscript: u32 = 0x242;
pub const kHIDUsage_Csmr_ACSuperscript: u32 = 0x243;
pub const kHIDUsage_Csmr_ACAllCaps: u32 = 0x244;
pub const kHIDUsage_Csmr_ACRotate: u32 = 0x245;
pub const kHIDUsage_Csmr_ACResize: u32 = 0x246;
pub const kHIDUsage_Csmr_ACFlipHorizontal: u32 = 0x247;
pub const kHIDUsage_Csmr_ACFlipVertical: u32 = 0x248;
pub const kHIDUsage_Csmr_ACMirrorHorizontal: u32 = 0x249;
pub const kHIDUsage_Csmr_ACMirrorVertical: u32 = 0x24A;
pub const kHIDUsage_Csmr_ACFontSelect: u32 = 0x24B;
pub const kHIDUsage_Csmr_ACFontColor: u32 = 0x24C;
pub const kHIDUsage_Csmr_ACFontSize: u32 = 0x24D;
pub const kHIDUsage_Csmr_ACJustifyLeft: u32 = 0x24E;
pub const kHIDUsage_Csmr_ACJustifyCenterH: u32 = 0x24F;
pub const kHIDUsage_Csmr_ACJustifyRight: u32 = 0x250;
pub const kHIDUsage_Csmr_ACJustifyBlockH: u32 = 0x251;
pub const kHIDUsage_Csmr_ACJustifyTop: u32 = 0x252;
pub const kHIDUsage_Csmr_ACJustifyCenterV: u32 = 0x253;
pub const kHIDUsage_Csmr_ACJustifyBottom: u32 = 0x254;
pub const kHIDUsage_Csmr_ACJustifyBlockV: u32 = 0x255;
pub const kHIDUsage_Csmr_ACIndentyDecrease: u32 = 0x256;
pub const kHIDUsage_Csmr_ACIndentyIncrease: u32 = 0x257;
pub const kHIDUsage_Csmr_ACNumberedList: u32 = 0x258;
pub const kHIDUsage_Csmr_ACRestartNumbering: u32 = 0x259;
pub const kHIDUsage_Csmr_ACBulletedList: u32 = 0x25A;
pub const kHIDUsage_Csmr_ACPromote: u32 = 0x25B;
pub const kHIDUsage_Csmr_ACDemote: u32 = 0x25C;
pub const kHIDUsage_Csmr_ACYes: u32 = 0x25D;
pub const kHIDUsage_Csmr_ACNo: u32 = 0x25E;
pub const kHIDUsage_Csmr_ACCancel: u32 = 0x25F;
pub const kHIDUsage_Csmr_ACCatalog: u32 = 0x260;
pub const kHIDUsage_Csmr_ACBuyOrCheckout: u32 = 0x261;
pub const kHIDUsage_Csmr_ACAddToCart: u32 = 0x262;
pub const kHIDUsage_Csmr_ACExpand: u32 = 0x263;
pub const kHIDUsage_Csmr_ACExpandAll: u32 = 0x264;
pub const kHIDUsage_Csmr_ACCollapse: u32 = 0x265;
pub const kHIDUsage_Csmr_ACCollapseAll: u32 = 0x266;
pub const kHIDUsage_Csmr_ACPrintPreview: u32 = 0x267;
pub const kHIDUsage_Csmr_ACPasteSpecial: u32 = 0x268;
pub const kHIDUsage_Csmr_ACInsertMode: u32 = 0x269;
pub const kHIDUsage_Csmr_ACDelete: u32 = 0x26A;
pub const kHIDUsage_Csmr_ACLock: u32 = 0x26B;
pub const kHIDUsage_Csmr_ACUnlock: u32 = 0x26C;
pub const kHIDUsage_Csmr_ACProtect: u32 = 0x26D;
pub const kHIDUsage_Csmr_ACUnprotect: u32 = 0x26E;
pub const kHIDUsage_Csmr_ACAttachComment: u32 = 0x26F;
pub const kHIDUsage_Csmr_ACDetachComment: u32 = 0x270;
pub const kHIDUsage_Csmr_ACViewComment: u32 = 0x271;
pub const kHIDUsage_Csmr_ACSelectWord: u32 = 0x272;
pub const kHIDUsage_Csmr_ACSelectSentence: u32 = 0x273;
pub const kHIDUsage_Csmr_ACSelectParagraph: u32 = 0x274;
pub const kHIDUsage_Csmr_ACSelectColumn: u32 = 0x275;
pub const kHIDUsage_Csmr_ACSelectRow: u32 = 0x276;
pub const kHIDUsage_Csmr_ACSelectTable: u32 = 0x277;
pub const kHIDUsage_Csmr_ACSelectObject: u32 = 0x278;
pub const kHIDUsage_Csmr_ACRedoOrRepeat: u32 = 0x279;
pub const kHIDUsage_Csmr_ACSort: u32 = 0x27A;
pub const kHIDUsage_Csmr_ACSortAscending: u32 = 0x27B;
pub const kHIDUsage_Csmr_ACSortDescending: u32 = 0x27C;
pub const kHIDUsage_Csmr_ACFilter: u32 = 0x27D;
pub const kHIDUsage_Csmr_ACSetClock: u32 = 0x27E;
pub const kHIDUsage_Csmr_ACViewClock: u32 = 0x27F;
pub const kHIDUsage_Csmr_ACSelectTimeZone: u32 = 0x280;
pub const kHIDUsage_Csmr_ACEditTimeZones: u32 = 0x281;
pub const kHIDUsage_Csmr_ACSetAlarm: u32 = 0x282;
pub const kHIDUsage_Csmr_ACClearAlarm: u32 = 0x283;
pub const kHIDUsage_Csmr_ACSnoozeAlarm: u32 = 0x284;
pub const kHIDUsage_Csmr_ACResetAlarm: u32 = 0x285;
pub const kHIDUsage_Csmr_ACSynchronize: u32 = 0x286;
pub const kHIDUsage_Csmr_ACSendOrReceive: u32 = 0x287;
pub const kHIDUsage_Csmr_ACSendTo: u32 = 0x288;
pub const kHIDUsage_Csmr_ACReply: u32 = 0x289;
pub const kHIDUsage_Csmr_ACReplyAll: u32 = 0x28A;
pub const kHIDUsage_Csmr_ACForwardMessage: u32 = 0x28B;
pub const kHIDUsage_Csmr_ACSend: u32 = 0x28C;
pub const kHIDUsage_Csmr_ACAttachFile: u32 = 0x28D;
pub const kHIDUsage_Csmr_ACUpload: u32 = 0x28E;
pub const kHIDUsage_Csmr_ACDownload: u32 = 0x28F;
pub const kHIDUsage_Csmr_ACSetBorders: u32 = 0x290;
pub const kHIDUsage_Csmr_ACInsertRow: u32 = 0x291;
pub const kHIDUsage_Csmr_ACInsertColumn: u32 = 0x292;
pub const kHIDUsage_Csmr_ACInsertFile: u32 = 0x293;
pub const kHIDUsage_Csmr_ACInsertPicture: u32 = 0x294;
pub const kHIDUsage_Csmr_ACInsertObject: u32 = 0x295;
pub const kHIDUsage_Csmr_ACInsertSymbol: u32 = 0x296;
pub const kHIDUsage_Csmr_ACSaveAndClose: u32 = 0x297;
pub const kHIDUsage_Csmr_ACRename: u32 = 0x298;
pub const kHIDUsage_Csmr_ACMerge: u32 = 0x299;
pub const kHIDUsage_Csmr_ACSplit: u32 = 0x29A;
pub const kHIDUsage_Csmr_ACDistributeH: u32 = 0x29B;
pub const kHIDUsage_Csmr_ACDistributeV: u32 = 0x29C;
pub const kHIDUsage_Csmr_ACKeyboardLayoutSelect: u32 = 0x29D;
// 0x29E - 0xFFFF Reserved
pub const kHIDUsage_Csmr_Reserved: u32 = 0xFFFF;

// Digitizer Page (0x0D)
pub const kHIDUsage_Dig_Digitizer: u32 = 0x01;
pub const kHIDUsage_Dig_Pen: u32 = 0x02;
pub const kHIDUsage_Dig_LightPen: u32 = 0x03;
pub const kHIDUsage_Dig_TouchScreen: u32 = 0x04;
pub const kHIDUsage_Dig_TouchPad: u32 = 0x05;
pub const kHIDUsage_Dig_WhiteBoard: u32 = 0x06;
pub const kHIDUsage_Dig_CoordinateMeasuringMachine: u32 = 0x07;
pub const kHIDUsage_Dig_3DDigitizer: u32 = 0x08;
pub const kHIDUsage_Dig_StereoPlotter: u32 = 0x09;
pub const kHIDUsage_Dig_ArticulatedArm: u32 = 0x0A;
pub const kHIDUsage_Dig_Armature: u32 = 0x0B;
pub const kHIDUsage_Dig_MultiplePointDigitizer: u32 = 0x0C;
pub const kHIDUsage_Dig_FreeSpaceWand: u32 = 0x0D;
pub const kHIDUsage_Dig_DeviceConfiguration: u32 = 0x0E;
// 0x0F - 0x1F Reserved
pub const kHIDUsage_Dig_Stylus: u32 = 0x20;
pub const kHIDUsage_Dig_Puck: u32 = 0x21;
pub const kHIDUsage_Dig_Finger: u32 = 0x22;
pub const kHIDUsage_Dig_DeviceSettings: u32 = 0x23;
pub const kHIDUsage_Dig_GestureCharacter: u32 = 0x24;
// 0x25 - 0x2F Reserved
pub const kHIDUsage_Dig_TipPressure: u32 = 0x30;
pub const kHIDUsage_Dig_BarrelPressure: u32 = 0x31;
pub const kHIDUsage_Dig_InRange: u32 = 0x32;
pub const kHIDUsage_Dig_Touch: u32 = 0x33;
pub const kHIDUsage_Dig_Untouch: u32 = 0x34;
pub const kHIDUsage_Dig_Tap: u32 = 0x35;
pub const kHIDUsage_Dig_Quality: u32 = 0x36;
pub const kHIDUsage_Dig_DataValid: u32 = 0x37;
pub const kHIDUsage_Dig_TransducerIndex: u32 = 0x38;
pub const kHIDUsage_Dig_TabletFunctionKeys: u32 = 0x39;
pub const kHIDUsage_Dig_ProgramChangeKeys: u32 = 0x3A;
pub const kHIDUsage_Dig_BatteryStrength: u32 = 0x3B;
pub const kHIDUsage_Dig_Invert: u32 = 0x3C;
pub const kHIDUsage_Dig_XTilt: u32 = 0x3D;
pub const kHIDUsage_Dig_YTilt: u32 = 0x3E;
pub const kHIDUsage_Dig_Azimuth: u32 = 0x3F;
pub const kHIDUsage_Dig_Altitude: u32 = 0x40;
pub const kHIDUsage_Dig_Twist: u32 = 0x41;
pub const kHIDUsage_Dig_TipSwitch: u32 = 0x42;
pub const kHIDUsage_Dig_SecondaryTipSwitch: u32 = 0x43;
pub const kHIDUsage_Dig_BarrelSwitch: u32 = 0x44;
pub const kHIDUsage_Dig_Eraser: u32 = 0x45;
pub const kHIDUsage_Dig_TabletPick: u32 = 0x46;
pub const kHIDUsage_Dig_TouchValid: u32 = 0x47;
pub const kHIDUsage_Dig_Width: u32 = 0x48;
pub const kHIDUsage_Dig_Height: u32 = 0x49;
// 0x4A - 0x50 Reserved
pub const kHIDUsage_Dig_ContactIdentifier: u32 = 0x51;
pub const kHIDUsage_Dig_DeviceMode: u32 = 0x52;
pub const kHIDUsage_Dig_DeviceIdentifier: u32 = 0x53;
pub const kHIDUsage_Dig_ContactCount: u32 = 0x54;
pub const kHIDUsage_Dig_ContactCountMaximum: u32 = 0x55;

// 0x56 - 0x5F Reserved
pub const kHIDUsage_Dig_GestureCharacterEnable: u32 = 0x60;
pub const kHIDUsage_Dig_GestureCharacterQuality: u32 = 0x61;
pub const kHIDUsage_Dig_GestureCharacterDataLength: u32 = 0x62;
pub const kHIDUsage_Dig_GestureCharacterData: u32 = 0x63;
pub const kHIDUsage_Dig_GestureCharacterEncoding: u32 = 0x64;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF8: u32 = 0x65;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF16LE: u32 = 0x66;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF16BE: u32 = 0x67;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF32LE: u32 = 0x68;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF32BE: u32 = 0x69;

// 0x70 - 0xFFFF Reserved
pub const kHIDUsage_Dig_Reserved: u32 = 0xFFFF;

// Physical Interface Device Page (0x0F)
pub const kHIDUsage_PID_PhysicalInterfaceDevice: u32 = 0x01;
// 0x02 - 0x1F Reserved
pub const kHIDUsage_PID_Normal: u32 = 0x20;
pub const kHIDUsage_PID_SetEffectReport: u32 = 0x21;
pub const kHIDUsage_PID_EffectBlockIndex: u32 = 0x22;
pub const kHIDUsage_PID_ParamBlockOffset: u32 = 0x23;
pub const kHIDUsage_PID_ROM_Flag: u32 = 0x24;
pub const kHIDUsage_PID_EffectType: u32 = 0x25;
pub const kHIDUsage_PID_ET_ConstantForce: u32 = 0x26;
pub const kHIDUsage_PID_ET_Ramp: u32 = 0x27;
pub const kHIDUsage_PID_ET_CustomForceData: u32 = 0x28;

// 0x29 - 0x2F Reserved
pub const kHIDUsage_PID_ET_Square: u32 = 0x30;
pub const kHIDUsage_PID_ET_Sine: u32 = 0x31;
pub const kHIDUsage_PID_ET_Triangle: u32 = 0x32;
pub const kHIDUsage_PID_ET_SawtoothUp: u32 = 0x33;
pub const kHIDUsage_PID_ET_SawtoothDown: u32 = 0x34;
// 0x35 - 0x3F Reserved
pub const kHIDUsage_PID_ET_Spring: u32 = 0x40;
pub const kHIDUsage_PID_ET_Damper: u32 = 0x41;
pub const kHIDUsage_PID_ET_Inertia: u32 = 0x42;
pub const kHIDUsage_PID_ET_Friction: u32 = 0x43;
// 0x44 - 0x4F Reserved
pub const kHIDUsage_PID_Duration: u32 = 0x50;
pub const kHIDUsage_PID_SamplePeriod: u32 = 0x51;
pub const kHIDUsage_PID_Gain: u32 = 0x52;
pub const kHIDUsage_PID_TriggerButton: u32 = 0x53;
pub const kHIDUsage_PID_TriggerRepeatInterval: u32 = 0x54;
pub const kHIDUsage_PID_AxesEnable: u32 = 0x55;
pub const kHIDUsage_PID_DirectionEnable: u32 = 0x56;
pub const kHIDUsage_PID_Direction: u32 = 0x57;
pub const kHIDUsage_PID_TypeSpecificBlockOffset: u32 = 0x58;
pub const kHIDUsage_PID_BlockType: u32 = 0x59;
pub const kHIDUsage_PID_SetEnvelopeReport: u32 = 0x5A;
pub const kHIDUsage_PID_AttackLevel: u32 = 0x5B;
pub const kHIDUsage_PID_AttackTime: u32 = 0x5C;
pub const kHIDUsage_PID_FadeLevel: u32 = 0x5D;
pub const kHIDUsage_PID_FadeTime: u32 = 0x5E;
pub const kHIDUsage_PID_SetConditionReport: u32 = 0x5F;

pub const kHIDUsage_PID_CP_Offset: u32 = 0x60;
pub const kHIDUsage_PID_PositiveCoefficient: u32 = 0x61;
pub const kHIDUsage_PID_NegativeCoefficient: u32 = 0x62;
pub const kHIDUsage_PID_PositiveSaturation: u32 = 0x63;
pub const kHIDUsage_PID_NegativeSaturation: u32 = 0x64;
pub const kHIDUsage_PID_DeadBand: u32 = 0x65;
pub const kHIDUsage_PID_DownloadForceSample: u32 = 0x66;
pub const kHIDUsage_PID_IsochCustomForceEnable: u32 = 0x67;
pub const kHIDUsage_PID_CustomForceDataReport: u32 = 0x68;
pub const kHIDUsage_PID_CustomForceData: u32 = 0x69;
pub const kHIDUsage_PID_CustomForceVendorDefinedData: u32 = 0x6A;
pub const kHIDUsage_PID_SetCustomForceReport: u32 = 0x6B;
pub const kHIDUsage_PID_CustomForceDataOffset: u32 = 0x6C;
pub const kHIDUsage_PID_SampleCount: u32 = 0x6D;
pub const kHIDUsage_PID_SetPeriodicReport: u32 = 0x6E;
pub const kHIDUsage_PID_Offset: u32 = 0x6F;

pub const kHIDUsage_PID_Magnitude: u32 = 0x70;
pub const kHIDUsage_PID_Phase: u32 = 0x71;
pub const kHIDUsage_PID_Period: u32 = 0x72;
pub const kHIDUsage_PID_SetConstantForceReport: u32 = 0x73;
pub const kHIDUsage_PID_SetRampForceReport: u32 = 0x74;
pub const kHIDUsage_PID_RampStart: u32 = 0x75;
pub const kHIDUsage_PID_RampEnd: u32 = 0x76;
pub const kHIDUsage_PID_EffectOperationReport: u32 = 0x77;
pub const kHIDUsage_PID_EffectOperation: u32 = 0x78;
pub const kHIDUsage_PID_OpEffectStart: u32 = 0x79;
pub const kHIDUsage_PID_OpEffectStartSolo: u32 = 0x7A;
pub const kHIDUsage_PID_OpEffectStop: u32 = 0x7B;
pub const kHIDUsage_PID_LoopCount: u32 = 0x7C;
pub const kHIDUsage_PID_DeviceGainReport: u32 = 0x7D;
pub const kHIDUsage_PID_DeviceGain: u32 = 0x7E;
pub const kHIDUsage_PID_PoolReport: u32 = 0x7F;

pub const kHIDUsage_PID_RAM_PoolSize: u32 = 0x80;
pub const kHIDUsage_PID_ROM_PoolSize: u32 = 0x81;
pub const kHIDUsage_PID_ROM_EffectBlockCount: u32 = 0x82;
pub const kHIDUsage_PID_SimultaneousEffectsMax: u32 = 0x83;
pub const kHIDUsage_PID_PoolAlignment: u32 = 0x84;
pub const kHIDUsage_PID_PoolMoveReport: u32 = 0x85;
pub const kHIDUsage_PID_MoveSource: u32 = 0x86;
pub const kHIDUsage_PID_MoveDestination: u32 = 0x87;
pub const kHIDUsage_PID_MoveLength: u32 = 0x88;
pub const kHIDUsage_PID_BlockLoadReport: u32 = 0x89;
// 0x8A Reserved
pub const kHIDUsage_PID_BlockLoadStatus: u32 = 0x8B;
pub const kHIDUsage_PID_BlockLoadSuccess: u32 = 0x8C;
pub const kHIDUsage_PID_BlockLoadFull: u32 = 0x8D;
pub const kHIDUsage_PID_BlockLoadError: u32 = 0x8E;
pub const kHIDUsage_PID_BlockHandle: u32 = 0x8F;

pub const kHIDUsage_PID_BlockFreeReport: u32 = 0x90;
pub const kHIDUsage_PID_TypeSpecificBlockHandle: u32 = 0x91;
pub const kHIDUsage_PID_StateReport: u32 = 0x92;
// 0x93 Reserved
pub const kHIDUsage_PID_EffectPlaying: u32 = 0x94;
pub const kHIDUsage_PID_DeviceControlReport: u32 = 0x95;
pub const kHIDUsage_PID_DeviceControl: u32 = 0x96;
pub const kHIDUsage_PID_DC_EnableActuators: u32 = 0x97;
pub const kHIDUsage_PID_DC_DisableActuators: u32 = 0x98;
pub const kHIDUsage_PID_DC_StopAllEffects: u32 = 0x99;
pub const kHIDUsage_PID_DC_DeviceReset: u32 = 0x9A;
pub const kHIDUsage_PID_DC_DevicePause: u32 = 0x9B;
pub const kHIDUsage_PID_DC_DeviceContinue: u32 = 0x9C;
// 0x9d - 0x9E Reserved
pub const kHIDUsage_PID_DevicePaused: u32 = 0x9F;

pub const kHIDUsage_PID_ActuatorsEnabled: u32 = 0xA0;
// 0xA1 - 0xA3 Reserved
pub const kHIDUsage_PID_SafetySwitch: u32 = 0xA4;
pub const kHIDUsage_PID_ActuatorOverrideSwitch: u32 = 0xA5;
pub const kHIDUsage_PID_ActuatorPower: u32 = 0xA6;
pub const kHIDUsage_PID_StartDelay: u32 = 0xA7;
pub const kHIDUsage_PID_ParameterBlockSize: u32 = 0xA8;
pub const kHIDUsage_PID_DeviceManagedPool: u32 = 0xA9;
pub const kHIDUsage_PID_SharedParameterBlocks: u32 = 0xAA;
pub const kHIDUsage_PID_CreateNewEffectReport: u32 = 0xAB;
pub const kHIDUsage_PID_RAM_PoolAvailable: u32 = 0xAC;

// 0xAD - 0xFFFF Reserved
pub const kHIDUsage_PID_Reserved: u32 = 0xFFFF;

// AlphanumericDisplay Page (0x14)
pub const kHIDUsage_AD_AlphanumericDisplay: u32 = 0x01;
// 0x02 - 0x1F Reserved
pub const kHIDUsage_AD_DisplayAttributesReport: u32 = 0x20;
pub const kHIDUsage_AD_ASCIICharacterSet: u32 = 0x21;
pub const kHIDUsage_AD_DataReadBack: u32 = 0x22;
pub const kHIDUsage_AD_FontReadBack: u32 = 0x23;
pub const kHIDUsage_AD_DisplayControlReport: u32 = 0x24;
pub const kHIDUsage_AD_ClearDisplay: u32 = 0x25;
pub const kHIDUsage_AD_DisplayEnable: u32 = 0x26;
pub const kHIDUsage_AD_ScreenSaverDelay: u32 = 0x27;
pub const kHIDUsage_AD_ScreenSaverEnable: u32 = 0x28;
pub const kHIDUsage_AD_VerticalScroll: u32 = 0x29;
pub const kHIDUsage_AD_HorizontalScroll: u32 = 0x2A;
pub const kHIDUsage_AD_CharacterReport: u32 = 0x2B;
pub const kHIDUsage_AD_DisplayData: u32 = 0x2C;
pub const kHIDUsage_AD_DisplayStatus: u32 = 0x2D;
pub const kHIDUsage_AD_StatNotReady: u32 = 0x2E;
pub const kHIDUsage_AD_StatReady: u32 = 0x2F;
pub const kHIDUsage_AD_ErrNotaloadablecharacter: u32 = 0x30;
pub const kHIDUsage_AD_ErrFontdatacannotberead: u32 = 0x31;
pub const kHIDUsage_AD_CursorPositionReport: u32 = 0x32;
pub const kHIDUsage_AD_Row: u32 = 0x33;
pub const kHIDUsage_AD_Column: u32 = 0x34;
pub const kHIDUsage_AD_Rows: u32 = 0x35;
pub const kHIDUsage_AD_Columns: u32 = 0x36;
pub const kHIDUsage_AD_CursorPixelPositioning: u32 = 0x37;
pub const kHIDUsage_AD_CursorMode: u32 = 0x38;
pub const kHIDUsage_AD_CursorEnable: u32 = 0x39;
pub const kHIDUsage_AD_CursorBlink: u32 = 0x3A;
pub const kHIDUsage_AD_FontReport: u32 = 0x3B;
pub const kHIDUsage_AD_FontData: u32 = 0x3C;
pub const kHIDUsage_AD_CharacterWidth: u32 = 0x3D;
pub const kHIDUsage_AD_CharacterHeight: u32 = 0x3E;
pub const kHIDUsage_AD_CharacterSpacingHorizontal: u32 = 0x3F;
pub const kHIDUsage_AD_CharacterSpacingVertical: u32 = 0x40;
pub const kHIDUsage_AD_UnicodeCharacterSet: u32 = 0x41;
// 0x42 - 0xFFFF Reserved
pub const kHIDUsage_AD_Reserved: u32 = 0xFFFF;

// Sensor Page (0x14)
pub const kHIDUsage_Snsr_Undefined: u32 = 0x00;
pub const kHIDUsage_Snsr_Sensor: u32 = 0x01;
// 0x02 - 0x0F Reserved
pub const kHIDUsage_Snsr_Biometric: u32 = 0x10;
pub const kHIDUsage_Snsr_Biometric_HumanPresence: u32 = 0x11;
pub const kHIDUsage_Snsr_Biometric_HumanProximity: u32 = 0x12;
pub const kHIDUsage_Snsr_Biometric_HumanTouch: u32 = 0x13;
// 0x14 - 0x1F Reserved
pub const kHIDUsage_Snsr_Electrical: u32 = 0x20;
pub const kHIDUsage_Snsr_Electrical_Capacitance: u32 = 0x21;
pub const kHIDUsage_Snsr_Electrical_Current: u32 = 0x22;
pub const kHIDUsage_Snsr_Electrical_Power: u32 = 0x23;
pub const kHIDUsage_Snsr_Electrical_Inductance: u32 = 0x24;
pub const kHIDUsage_Snsr_Electrical_Resistance: u32 = 0x25;
pub const kHIDUsage_Snsr_Electrical_Voltage: u32 = 0x26;
pub const kHIDUsage_Snsr_Electrical_Potentiometer: u32 = 0x27;
pub const kHIDUsage_Snsr_Electrical_Frequency: u32 = 0x28;
pub const kHIDUsage_Snsr_Electrical_Period: u32 = 0x29;
// 0x2A - 0x2F Reserved
pub const kHIDUsage_Snsr_Environmental: u32 = 0x30;
pub const kHIDUsage_Snsr_Environmental_AtmosphericPressure: u32 = 0x31;
pub const kHIDUsage_Snsr_Environmental_Humidity: u32 = 0x32;
pub const kHIDUsage_Snsr_Environmental_Temperature: u32 = 0x33;
pub const kHIDUsage_Snsr_Environmental_WindDirection: u32 = 0x34;
pub const kHIDUsage_Snsr_Environmental_WindSpeed: u32 = 0x35;
// 0x36 - 0x3F Reserved
pub const kHIDUsage_Snsr_Light: u32 = 0x40;
pub const kHIDUsage_Snsr_Light_AmbientLight: u32 = 0x41;
pub const kHIDUsage_Snsr_Light_ConsumerInfrared: u32 = 0x42;
// 0x43 - 0x4F Reserved
pub const kHIDUsage_Snsr_Location: u32 = 0x50;
pub const kHIDUsage_Snsr_Location_Broadcast: u32 = 0x51;
pub const kHIDUsage_Snsr_Location_DeadReckoning: u32 = 0x52;
pub const kHIDUsage_Snsr_Location_GPS: u32 = 0x53;
pub const kHIDUsage_Snsr_Location_Lookup: u32 = 0x54;
pub const kHIDUsage_Snsr_Location_Other: u32 = 0x55;
pub const kHIDUsage_Snsr_Location_Static: u32 = 0x56;
pub const kHIDUsage_Snsr_Location_Triangulation: u32 = 0x57;
// 0x58 - 0x5F Reserved
pub const kHIDUsage_Snsr_Mechanical: u32 = 0x60;
pub const kHIDUsage_Snsr_Mechanical_BooleanSwitch: u32 = 0x61;
pub const kHIDUsage_Snsr_Mechanical_BooleanSwitchArray: u32 = 0x62;
pub const kHIDUsage_Snsr_Mechanical_MultivalueSwitch: u32 = 0x63;
pub const kHIDUsage_Snsr_Mechanical_Force: u32 = 0x64;
pub const kHIDUsage_Snsr_Mechanical_Pressure: u32 = 0x65;
pub const kHIDUsage_Snsr_Mechanical_Strain: u32 = 0x66;
pub const kHIDUsage_Snsr_Mechanical_Weight: u32 = 0x67;
pub const kHIDUsage_Snsr_Mechanical_HapticVibrator: u32 = 0x68;
pub const kHIDUsage_Snsr_Mechanical_HallEffectSwitch: u32 = 0x69;
// 0x6A - 0x6F Reserved
pub const kHIDUsage_Snsr_Motion: u32 = 0x70;
pub const kHIDUsage_Snsr_Motion_Accelerometer1D: u32 = 0x71;
pub const kHIDUsage_Snsr_Motion_Accelerometer2D: u32 = 0x72;
pub const kHIDUsage_Snsr_Motion_Accelerometer3D: u32 = 0x73;
pub const kHIDUsage_Snsr_Motion_Gyrometer1D: u32 = 0x74;
pub const kHIDUsage_Snsr_Motion_Gyrometer2D: u32 = 0x75;
pub const kHIDUsage_Snsr_Motion_Gyrometer3D: u32 = 0x76;
pub const kHIDUsage_Snsr_Motion_MotionDetector: u32 = 0x77;
pub const kHIDUsage_Snsr_Motion_Speedometer: u32 = 0x78;
pub const kHIDUsage_Snsr_Motion_Accelerometer: u32 = 0x79;
pub const kHIDUsage_Snsr_Motion_Gyrometer: u32 = 0x7A;
// 0x7B - 0x7F Reserved
pub const kHIDUsage_Snsr_Orientation: u32 = 0x80;
pub const kHIDUsage_Snsr_Orientation_Compass1D: u32 = 0x81;
pub const kHIDUsage_Snsr_Orientation_Compass2D: u32 = 0x82;
pub const kHIDUsage_Snsr_Orientation_Compass3D: u32 = 0x83;
pub const kHIDUsage_Snsr_Orientation_Inclinometer1D: u32 = 0x84;
pub const kHIDUsage_Snsr_Orientation_Inclinometer2D: u32 = 0x85;
pub const kHIDUsage_Snsr_Orientation_Inclinometer3D: u32 = 0x86;
pub const kHIDUsage_Snsr_Orientation_Distance1D: u32 = 0x87;
pub const kHIDUsage_Snsr_Orientation_Distance2D: u32 = 0x88;
pub const kHIDUsage_Snsr_Orientation_Distance3D: u32 = 0x89;
pub const kHIDUsage_Snsr_Orientation_DeviceOrientation: u32 = 0x8A;
pub const kHIDUsage_Snsr_Orientation_CompassD: u32 = 0x8B;
pub const kHIDUsage_Snsr_Orientation_InclinometerD: u32 = 0x8C;
pub const kHIDUsage_Snsr_Orientation_DistanceD: u32 = 0x8D;
// 0x8E - 0x8F Reserved
pub const kHIDUsage_Snsr_Scanner: u32 = 0x90;
pub const kHIDUsage_Snsr_Scanner_Barcode: u32 = 0x91;
pub const kHIDUsage_Snsr_Scanner_RFID: u32 = 0x92;
pub const kHIDUsage_Snsr_Scanner_NFC: u32 = 0x93;
// 0x94 - 0x9F Reserved
pub const kHIDUsage_Snsr_Time: u32 = 0xA0;
pub const kHIDUsage_Snsr_Time_AlarmTimer: u32 = 0xA1;
pub const kHIDUsage_Snsr_Time_RealTimeClock: u32 = 0xA2;
// 0xA3 - 0xDF Reserved
pub const kHIDUsage_Snsr_Other: u32 = 0xE0;
pub const kHIDUsage_Snsr_Other_Custom: u32 = 0xE1;
pub const kHIDUsage_Snsr_Other_Generic: u32 = 0xE2;
pub const kHIDUsage_Snsr_Other_GenericEnumerator: u32 = 0xE3;
// 0xE4 - 0xEF Reserved
// 0xF0 - 0xFF Vendor Reserved

// Common Sensor Type Data Fields
pub const kHIDUsage_Snsr_Modifier_None: u32 = 0x0;
pub const kHIDUsage_Snsr_Modifier_ChangeSensitivityAbsolute: u32 = 0x1;
pub const kHIDUsage_Snsr_Modifier_Max: u32 = 0x2;
pub const kHIDUsage_Snsr_Modifier_Min: u32 = 0x3;
pub const kHIDUsage_Snsr_Modifier_Accuracy: u32 = 0x4;
pub const kHIDUsage_Snsr_Modifier_Resolution: u32 = 0x5;
pub const kHIDUsage_Snsr_Modifier_ThresholdHigh: u32 = 0x6;
pub const kHIDUsage_Snsr_Modifier_ThresholdLow: u32 = 0x7;
pub const kHIDUsage_Snsr_Modifier_CalibrationOffset: u32 = 0x8;
pub const kHIDUsage_Snsr_Modifier_CalibrationMultiplier: u32 = 0x9;
pub const kHIDUsage_Snsr_Modifier_ReportInterval: u32 = 0xA;
pub const kHIDUsage_Snsr_Modifier_FrequencyMax: u32 = 0xB;
pub const kHIDUsage_Snsr_Modifier_PeriodMax: u32 = 0xC;
pub const kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRange: u32 = 0xD;
pub const kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRelative: u32 = 0xE;
pub const kHIDUsage_Snsr_Modifier_VendorDefined: u32 = 0xF;

// Event Usages
pub const kHIDUsage_Snsr_Event: u32 = 0x0200;
pub const kHIDUsage_Snsr_Event_SensorState: u32 = 0x0201;
pub const kHIDUsage_Snsr_Event_SensorEvent: u32 = 0x0202;
// 0x0203 - 0x02FF Event Reserved

pub const kHIDUsage_Snsr_Event_SensorState_Undefined: u32 = 0x0800;
pub const kHIDUsage_Snsr_Event_SensorState_Ready: u32 = 0x0801;
pub const kHIDUsage_Snsr_Event_SensorState_NotAvailable: u32 = 0x0802;
pub const kHIDUsage_Snsr_Event_SensorState_NoData: u32 = 0x0803;
pub const kHIDUsage_Snsr_Event_SensorState_Initializing: u32 = 0x0804;
pub const kHIDUsage_Snsr_Event_SensorState_AccessDenied: u32 = 0x0805;
pub const kHIDUsage_Snsr_Event_SensorState_Error: u32 = 0x0806;
// 0x0807 - 0x080F Reserved

pub const kHIDUsage_Snsr_Event_SensorEvent_Unknown: u32 = 0x0810;
pub const kHIDUsage_Snsr_Event_SensorEvent_StateChanged: u32 = 0x0811;
pub const kHIDUsage_Snsr_Event_SensorEvent_PropertyChanged: u32 = 0x0812;
pub const kHIDUsage_Snsr_Event_SensorEvent_DataUpdated: u32 = 0x0813;
pub const kHIDUsage_Snsr_Event_SensorEvent_PollResponse: u32 = 0x0814;
pub const kHIDUsage_Snsr_Event_SensorEvent_ChangeSensitivity: u32 = 0x0815;
pub const kHIDUsage_Snsr_Event_SensorEvent_RangeMaxReached: u32 = 0x0816;
pub const kHIDUsage_Snsr_Event_SensorEvent_RangeMinReached: u32 = 0x0817;
pub const kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossUp: u32 = 0x0818;
pub const kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossDown: u32 = 0x0819;
pub const kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossUp: u32 = 0x081A;
pub const kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossDown: u32 = 0x081B;
pub const kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossUp: u32 = 0x081C;
pub const kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossDown: u32 = 0x081D;
pub const kHIDUsage_Snsr_Event_SensorEvent_PeriodExceeded: u32 = 0x081E;
pub const kHIDUsage_Snsr_Event_SensorEvent_FrequencyExceeded: u32 = 0x081F;
pub const kHIDUsage_Snsr_Event_SensorEvent_ComplexTrigger: u32 = 0x0820;
// 0x0821 - 0x082F Reserved

// Property Usages
pub const kHIDUsage_Snsr_Property: u32 = 0x0300;
pub const kHIDUsage_Snsr_Property_FriendlyName: u32 = 0x0301;
pub const kHIDUsage_Snsr_Property_PersistentUniqueID: u32 = 0x0302;
pub const kHIDUsage_Snsr_Property_SensorStatus: u32 = 0x0303;
pub const kHIDUsage_Snsr_Property_MinimumReportInterval: u32 = 0x0304;
pub const kHIDUsage_Snsr_Property_Manufacturer: u32 = 0x0305;
pub const kHIDUsage_Snsr_Property_Model: u32 = 0x0306;
pub const kHIDUsage_Snsr_Property_SerialNumber: u32 = 0x0307;
pub const kHIDUsage_Snsr_Property_Description: u32 = 0x0308;
pub const kHIDUsage_Snsr_Property_ConnectionType: u32 = 0x0309;
pub const kHIDUsage_Snsr_Property_DevicePath: u32 = 0x030A;
pub const kHIDUsage_Snsr_Property_HardwareRevision: u32 = 0x030B;
pub const kHIDUsage_Snsr_Property_FirmwareVersion: u32 = 0x030C;
pub const kHIDUsage_Snsr_Property_ReleaseData: u32 = 0x030D;
pub const kHIDUsage_Snsr_Property_ReportInterval: u32 = 0x030E;
pub const kHIDUsage_Snsr_Property_ChangeSensitivityAbsolute: u32 = 0x030F;
pub const kHIDUsage_Snsr_Property_ChangeSensitivityPercentRange: u32 = 0x0310;
pub const kHIDUsage_Snsr_Property_ChangeSensitivityPercentRelative: u32 = 0x0311;
pub const kHIDUsage_Snsr_Property_Accuracy: u32 = 0x0312;
pub const kHIDUsage_Snsr_Property_Resolution: u32 = 0x0313;
pub const kHIDUsage_Snsr_Property_Maximum: u32 = 0x0314;
pub const kHIDUsage_Snsr_Property_Minimum: u32 = 0x0315;
pub const kHIDUsage_Snsr_Property_ReportingState: u32 = 0x0316;
pub const kHIDUsage_Snsr_Property_SamplingRate: u32 = 0x0317;
pub const kHIDUsage_Snsr_Property_ResponseCurve: u32 = 0x0318;
pub const kHIDUsage_Snsr_Property_PowerState: u32 = 0x0319;
// 0x031A - 0x03FF Reserved

pub const kHIDUsage_Snsr_Property_ConnectionType_Integrated: u32 = 0x0830;
pub const kHIDUsage_Snsr_Property_ConnectionType_Attached: u32 = 0x0831;
pub const kHIDUsage_Snsr_Property_ConnectionType_External: u32 = 0x0832;
// 0x0833 - 0x083F Reserved
pub const kHIDUsage_Snsr_Property_ReportingState_NoEvents: u32 = 0x0840;
pub const kHIDUsage_Snsr_Property_ReportingState_AllEvents: u32 = 0x0841;
pub const kHIDUsage_Snsr_Property_ReportingState_ThresholdEvents: u32 = 0x0842;
pub const kHIDUsage_Snsr_Property_ReportingState_WakeNoEvents: u32 = 0x0843;
pub const kHIDUsage_Snsr_Property_ReportingState_WakeAllEvents: u32 = 0x0844;
pub const kHIDUsage_Snsr_Property_ReportingState_WakeThresholdEvents: u32 = 0x0845;
// 0x0846 - 0x084F Reserved
pub const kHIDUsage_Snsr_Property_PowerState_Undefined: u32 = 0x0850;
pub const kHIDUsage_Snsr_Property_PowerState_D0_FullPower: u32 = 0x0851;
pub const kHIDUsage_Snsr_Property_PowerState_D1_LowPower: u32 = 0x0852;
pub const kHIDUsage_Snsr_Property_PowerState_D2_Standby: u32 = 0x0853;
pub const kHIDUsage_Snsr_Property_PowerState_D3_Sleep: u32 = 0x0854;
pub const kHIDUsage_Snsr_Property_PowerState_D4_PowerOff: u32 = 0x0855;
// 0x0855 - 0x085F Reserved

// Power Device Page (0x84)
pub const kHIDUsage_PD_Undefined: u32 = 0x00;
pub const kHIDUsage_PD_iName: u32 = 0x01;
pub const kHIDUsage_PD_PresentStatus: u32 = 0x02;
pub const kHIDUsage_PD_ChangedStatus: u32 = 0x03;
pub const kHIDUsage_PD_UPS: u32 = 0x04;
pub const kHIDUsage_PD_PowerSupply: u32 = 0x05;
// Reserved 0x06 - 0x0F
pub const kHIDUsage_PD_BatterySystem: u32 = 0x10;
pub const kHIDUsage_PD_BatterySystemID: u32 = 0x11;
pub const kHIDUsage_PD_Battery: u32 = 0x12;
pub const kHIDUsage_PD_BatteryID: u32 = 0x13;
pub const kHIDUsage_PD_Charger: u32 = 0x14;
pub const kHIDUsage_PD_ChargerID: u32 = 0x15;
pub const kHIDUsage_PD_PowerConverter: u32 = 0x16;
pub const kHIDUsage_PD_PowerConverterID: u32 = 0x17;
pub const kHIDUsage_PD_OutletSystem: u32 = 0x18;
pub const kHIDUsage_PD_OutletSystemID: u32 = 0x19;
pub const kHIDUsage_PD_Input: u32 = 0x1A;
pub const kHIDUsage_PD_InputID: u32 = 0x1B;
pub const kHIDUsage_PD_Output: u32 = 0x1C;
pub const kHIDUsage_PD_OutputID: u32 = 0x1D;
pub const kHIDUsage_PD_Flow: u32 = 0x1E;
pub const kHIDUsage_PD_FlowID: u32 = 0x1F;
pub const kHIDUsage_PD_Outlet: u32 = 0x20;
pub const kHIDUsage_PD_OutletID: u32 = 0x21;
pub const kHIDUsage_PD_Gang: u32 = 0x22;
pub const kHIDUsage_PD_GangID: u32 = 0x23;
pub const kHIDUsage_PD_PowerSummary: u32 = 0x24;
pub const kHIDUsage_PD_PowerSummaryID: u32 = 0x25;
// Reserved 0x26 - 0x2F
pub const kHIDUsage_PD_Voltage: u32 = 0x30;
pub const kHIDUsage_PD_Current: u32 = 0x31;
pub const kHIDUsage_PD_Frequency: u32 = 0x32;
pub const kHIDUsage_PD_ApparentPower: u32 = 0x33;
pub const kHIDUsage_PD_ActivePower: u32 = 0x34;
pub const kHIDUsage_PD_PercentLoad: u32 = 0x35;
pub const kHIDUsage_PD_Temperature: u32 = 0x36;
pub const kHIDUsage_PD_Humidity: u32 = 0x37;
pub const kHIDUsage_PD_BadCount: u32 = 0x38;
// Reserved 0x39 - 0x3F
pub const kHIDUsage_PD_ConfigVoltage: u32 = 0x40;
pub const kHIDUsage_PD_ConfigCurrent: u32 = 0x41;
pub const kHIDUsage_PD_ConfigFrequency: u32 = 0x42;
pub const kHIDUsage_PD_ConfigApparentPower: u32 = 0x43;
pub const kHIDUsage_PD_ConfigActivePower: u32 = 0x44;
pub const kHIDUsage_PD_ConfigPercentLoad: u32 = 0x45;
pub const kHIDUsage_PD_ConfigTemperature: u32 = 0x46;
pub const kHIDUsage_PD_ConfigHumidity: u32 = 0x47;
// Reserved 0x48 - 0x4F
pub const kHIDUsage_PD_SwitchOnControl: u32 = 0x50;
pub const kHIDUsage_PD_SwitchOffControl: u32 = 0x51;
pub const kHIDUsage_PD_ToggleControl: u32 = 0x52;
pub const kHIDUsage_PD_LowVoltageTransfer: u32 = 0x53;
pub const kHIDUsage_PD_HighVoltageTransfer: u32 = 0x54;
pub const kHIDUsage_PD_DelayBeforeReboot: u32 = 0x55;
pub const kHIDUsage_PD_DelayBeforeStartup: u32 = 0x56;
pub const kHIDUsage_PD_DelayBeforeShutdown: u32 = 0x57;
pub const kHIDUsage_PD_Test: u32 = 0x58;
pub const kHIDUsage_PD_ModuleReset: u32 = 0x59;
pub const kHIDUsage_PD_AudibleAlarmControl: u32 = 0x5A;
// Reserved 0x5B - 0x5F
pub const kHIDUsage_PD_Present: u32 = 0x60;
pub const kHIDUsage_PD_Good: u32 = 0x61;
pub const kHIDUsage_PD_InternalFailure: u32 = 0x62;
pub const kHIDUsage_PD_VoltageOutOfRange: u32 = 0x63;
pub const kHIDUsage_PD_FrequencyOutOfRange: u32 = 0x64;
pub const kHIDUsage_PD_Overload: u32 = 0x65;
pub const kHIDUsage_PD_OverCharged: u32 = 0x66;
pub const kHIDUsage_PD_OverTemperature: u32 = 0x67;
pub const kHIDUsage_PD_ShutdownRequested: u32 = 0x68;
pub const kHIDUsage_PD_ShutdownImminent: u32 = 0x69;
// Reserved 0x6A
pub const kHIDUsage_PD_SwitchOnOff: u32 = 0x6B;
pub const kHIDUsage_PD_Switchable: u32 = 0x6C;
pub const kHIDUsage_PD_Used: u32 = 0x6D;
pub const kHIDUsage_PD_Boost: u32 = 0x6E;
pub const kHIDUsage_PD_Buck: u32 = 0x6F;
pub const kHIDUsage_PD_Initialized: u32 = 0x70;
pub const kHIDUsage_PD_Tested: u32 = 0x71;
pub const kHIDUsage_PD_AwaitingPower: u32 = 0x72;
pub const kHIDUsage_PD_CommunicationLost: u32 = 0x73;
// Reserved 0x74 - 0xFC
pub const kHIDUsage_PD_iManufacturer: u32 = 0xFD;
pub const kHIDUsage_PD_iProduct: u32 = 0xFE;
pub const kHIDUsage_PD_iserialNumber: u32 = 0xFF;

// Battery System Page (x85)
pub const kHIDUsage_BS_Undefined: u32 = 0x00;
pub const kHIDUsage_BS_SMBBatteryMode: u32 = 0x01;
pub const kHIDUsage_BS_SMBBatteryStatus: u32 = 0x02;
pub const kHIDUsage_BS_SMBAlarmWarning: u32 = 0x03;
pub const kHIDUsage_BS_SMBChargerMode: u32 = 0x04;
pub const kHIDUsage_BS_SMBChargerStatus: u32 = 0x05;
pub const kHIDUsage_BS_SMBChargerSpecInfo: u32 = 0x06;
pub const kHIDUsage_BS_SMBSelectorState: u32 = 0x07;
pub const kHIDUsage_BS_SMBSelectorPresets: u32 = 0x08;
pub const kHIDUsage_BS_SMBSelectorInfo: u32 = 0x09;
// Reserved 0x0A - 0x0F
pub const kHIDUsage_BS_OptionalMfgFunction1: u32 = 0x10;
pub const kHIDUsage_BS_OptionalMfgFunction2: u32 = 0x11;
pub const kHIDUsage_BS_OptionalMfgFunction3: u32 = 0x12;
pub const kHIDUsage_BS_OptionalMfgFunction4: u32 = 0x13;
pub const kHIDUsage_BS_OptionalMfgFunction5: u32 = 0x14;
pub const kHIDUsage_BS_ConnectionToSMBus: u32 = 0x15;
pub const kHIDUsage_BS_OutputConnection: u32 = 0x16;
pub const kHIDUsage_BS_ChargerConnection: u32 = 0x17;
pub const kHIDUsage_BS_BatteryInsertion: u32 = 0x18;
pub const kHIDUsage_BS_Usenext: u32 = 0x19;
pub const kHIDUsage_BS_OKToUse: u32 = 0x1A;
pub const kHIDUsage_BS_BatterySupported: u32 = 0x1B;
pub const kHIDUsage_BS_SelectorRevision: u32 = 0x1C;
pub const kHIDUsage_BS_ChargingIndicator: u32 = 0x1D;
// Reserved 0x1E - 0x27
pub const kHIDUsage_BS_ManufacturerAccess: u32 = 0x28;
pub const kHIDUsage_BS_RemainingCapacityLimit: u32 = 0x29;
pub const kHIDUsage_BS_RemainingTimeLimit: u32 = 0x2A;
pub const kHIDUsage_BS_AtRate: u32 = 0x2B;
pub const kHIDUsage_BS_CapacityMode: u32 = 0x2C;
pub const kHIDUsage_BS_BroadcastToCharger: u32 = 0x2D;
pub const kHIDUsage_BS_PrimaryBattery: u32 = 0x2E;
pub const kHIDUsage_BS_ChargeController: u32 = 0x2F;
// Reserved 0x30 - 0x3F
pub const kHIDUsage_BS_TerminateCharge: u32 = 0x40;
pub const kHIDUsage_BS_TerminateDischarge: u32 = 0x41;
pub const kHIDUsage_BS_BelowRemainingCapacityLimit: u32 = 0x42;
pub const kHIDUsage_BS_RemainingTimeLimitExpired: u32 = 0x43;
pub const kHIDUsage_BS_Charging: u32 = 0x44;
pub const kHIDUsage_BS_Discharging: u32 = 0x45;
pub const kHIDUsage_BS_FullyCharged: u32 = 0x46;
pub const kHIDUsage_BS_FullyDischarged: u32 = 0x47;
pub const kHIDUsage_BS_ConditioningFlag: u32 = 0x48;
pub const kHIDUsage_BS_AtRateOK: u32 = 0x49;
pub const kHIDUsage_BS_SMBErrorCode: u32 = 0x4A;
pub const kHIDUsage_BS_NeedReplacement: u32 = 0x4B;
// Reserved 0x4C - 0x5F
pub const kHIDUsage_BS_AtRateTimeToFull: u32 = 0x60;
pub const kHIDUsage_BS_AtRateTimeToEmpty: u32 = 0x61;
pub const kHIDUsage_BS_AverageCurrent: u32 = 0x62;
pub const kHIDUsage_BS_Maxerror: u32 = 0x63;
pub const kHIDUsage_BS_RelativeStateOfCharge: u32 = 0x64;
pub const kHIDUsage_BS_AbsoluteStateOfCharge: u32 = 0x65;
pub const kHIDUsage_BS_RemainingCapacity: u32 = 0x66;
pub const kHIDUsage_BS_FullChargeCapacity: u32 = 0x67;
pub const kHIDUsage_BS_RunTimeToEmpty: u32 = 0x68;
pub const kHIDUsage_BS_AverageTimeToEmpty: u32 = 0x69;
pub const kHIDUsage_BS_AverageTimeToFull: u32 = 0x6A;
pub const kHIDUsage_BS_CycleCount: u32 = 0x6B;
// Reserved 0x6C - 0x7F
pub const kHIDUsage_BS_BattPackModelLevel: u32 = 0x80;
pub const kHIDUsage_BS_InternalChargeController: u32 = 0x81;
pub const kHIDUsage_BS_PrimaryBatterySupport: u32 = 0x82;
pub const kHIDUsage_BS_DesignCapacity: u32 = 0x83;
pub const kHIDUsage_BS_SpecificationInfo: u32 = 0x84;
pub const kHIDUsage_BS_ManufacturerDate: u32 = 0x85;
pub const kHIDUsage_BS_SerialNumber: u32 = 0x86;
pub const kHIDUsage_BS_iManufacturerName: u32 = 0x87;
pub const kHIDUsage_BS_iDevicename: u32 = 0x88;
pub const kHIDUsage_BS_iDeviceChemistry: u32 = 0x89;
pub const kHIDUsage_BS_ManufacturerData: u32 = 0x8A;
pub const kHIDUsage_BS_Rechargable: u32 = 0x8B;
pub const kHIDUsage_BS_WarningCapacityLimit: u32 = 0x8C;
pub const kHIDUsage_BS_CapacityGranularity1: u32 = 0x8D;
pub const kHIDUsage_BS_CapacityGranularity2: u32 = 0x8E;
pub const kHIDUsage_BS_iOEMInformation: u32 = 0x8F;
// Reserved 0x90 - 0xBF
pub const kHIDUsage_BS_InhibitCharge: u32 = 0xC0;
pub const kHIDUsage_BS_EnablePolling: u32 = 0xC1;
pub const kHIDUsage_BS_ResetToZero: u32 = 0xC2;
// Reserved 0xC3 - 0xCF
pub const kHIDUsage_BS_ACPresent: u32 = 0xD0;
pub const kHIDUsage_BS_BatteryPresent: u32 = 0xD1;
pub const kHIDUsage_BS_PowerFail: u32 = 0xD2;
pub const kHIDUsage_BS_AlarmInhibited: u32 = 0xD3;
pub const kHIDUsage_BS_ThermistorUnderRange: u32 = 0xD4;
pub const kHIDUsage_BS_ThermistorHot: u32 = 0xD5;
pub const kHIDUsage_BS_ThermistorCold: u32 = 0xD6;
pub const kHIDUsage_BS_ThermistorOverRange: u32 = 0xD7;
pub const kHIDUsage_BS_VoltageOutOfRange: u32 = 0xD8;
pub const kHIDUsage_BS_CurrentOutOfRange: u32 = 0xD9;
pub const kHIDUsage_BS_CurrentNotRegulated: u32 = 0xDA;
pub const kHIDUsage_BS_VoltageNotRegulated: u32 = 0xDB;
pub const kHIDUsage_BS_MasterMode: u32 = 0xDC;
// Reserved 0xDD - 0xEF
pub const kHIDUsage_BS_ChargerSelectorSupport: u32 = 0xF0;
pub const kHIDUsage_BS_ChargerSpec: u32 = 0xF1;
pub const kHIDUsage_BS_Level2: u32 = 0xF2;
pub const kHIDUsage_BS_Level3: u32 = 0xF3;
// Reserved 0xF2 - 0xFF

// Bar Code Scanner Page (0x8C)
pub const kHIDUsage_BCS_Undefined: u32 = 0x00;
pub const kHIDUsage_BCS_BadgeReader: u32 = 0x01;
pub const kHIDUsage_BCS_BarCodeScanner: u32 = 0x02;
pub const kHIDUsage_BCS_DumbBarCodeScanner: u32 = 0x03;
pub const kHIDUsage_BCS_CordlessScannerBase: u32 = 0x04;
pub const kHIDUsage_BCS_BarCodeScannerCradle: u32 = 0x05;
// Reserved 0x06 - 0x0F
pub const kHIDUsage_BCS_AttributeReport: u32 = 0x10;
pub const kHIDUsage_BCS_SettingsReport: u32 = 0x11;
pub const kHIDUsage_BCS_ScannedDataReport: u32 = 0x12;
pub const kHIDUsage_BCS_RawScannedDataReport: u32 = 0x13;
pub const kHIDUsage_BCS_TriggerReport: u32 = 0x14;
pub const kHIDUsage_BCS_StatusReport: u32 = 0x15;
pub const kHIDUsage_BCS_UPC_EANControlReport: u32 = 0x16;
pub const kHIDUsage_BCS_EAN2_3LabelControlReport: u32 = 0x17;
pub const kHIDUsage_BCS_Code39ControlReport: u32 = 0x18;
pub const kHIDUsage_BCS_Interleaved2of5ControlReport: u32 = 0x19;
pub const kHIDUsage_BCS_Standard2of5ControlReport: u32 = 0x1A;
pub const kHIDUsage_BCS_MSIPlesseyControlReport: u32 = 0x1B;
pub const kHIDUsage_BCS_CodabarControlReport: u32 = 0x1C;
pub const kHIDUsage_BCS_Code128ControlReport: u32 = 0x1D;
pub const kHIDUsage_BCS_Misc1DControlReport: u32 = 0x1E;
pub const kHIDUsage_BCS_2DControlReport: u32 = 0x1F;
// Reserved 0x20 - 0x2F
pub const kHIDUsage_BCS_Aiming_PointerMide: u32 = 0x30;
pub const kHIDUsage_BCS_BarCodePresentSensor: u32 = 0x31;
pub const kHIDUsage_BCS_Class1ALaser: u32 = 0x32;
pub const kHIDUsage_BCS_Class2Laser: u32 = 0x33;
pub const kHIDUsage_BCS_HeaterPresent: u32 = 0x34;
pub const kHIDUsage_BCS_ContactScanner: u32 = 0x35;
pub const kHIDUsage_BCS_ElectronicArticleSurveillanceNotification: u32 = 0x36;
pub const kHIDUsage_BCS_ConstantElectronicArticleSurveillance: u32 = 0x37;
pub const kHIDUsage_BCS_ErrorIndication: u32 = 0x38;
pub const kHIDUsage_BCS_FixedBeeper: u32 = 0x39;
pub const kHIDUsage_BCS_GoodDecodeIndication: u32 = 0x3A;
pub const kHIDUsage_BCS_HandsFreeScanning: u32 = 0x3B;
pub const kHIDUsage_BCS_IntrinsicallySafe: u32 = 0x3C;
pub const kHIDUsage_BCS_KlasseEinsLaser: u32 = 0x3D;
pub const kHIDUsage_BCS_LongRangeScanner: u32 = 0x3E;
pub const kHIDUsage_BCS_MirrorSpeedControl: u32 = 0x3F;
pub const kHIDUsage_BCS_NotOnFileIndication: u32 = 0x40;
pub const kHIDUsage_BCS_ProgrammableBeeper: u32 = 0x41;
pub const kHIDUsage_BCS_Triggerless: u32 = 0x42;
pub const kHIDUsage_BCS_Wand: u32 = 0x43;
pub const kHIDUsage_BCS_WaterResistant: u32 = 0x44;
pub const kHIDUsage_BCS_MultiRangeScanner: u32 = 0x45;
pub const kHIDUsage_BCS_ProximitySensor: u32 = 0x46;
// Reserved 0x47 - 0x4C
pub const kHIDUsage_BCS_FragmentDecoding: u32 = 0x4D;
pub const kHIDUsage_BCS_ScannerReadConfidence: u32 = 0x4E;
pub const kHIDUsage_BCS_DataPrefix: u32 = 0x4F;
pub const kHIDUsage_BCS_PrefixAIMI: u32 = 0x50;
pub const kHIDUsage_BCS_PrefixNone: u32 = 0x51;
pub const kHIDUsage_BCS_PrefixProprietary: u32 = 0x52;
// Reserved 0x53 - 0x54
pub const kHIDUsage_BCS_ActiveTime: u32 = 0x55;
pub const kHIDUsage_BCS_AimingLaserPattern: u32 = 0x56;
pub const kHIDUsage_BCS_BarCodePresent: u32 = 0x57;
pub const kHIDUsage_BCS_BeeperState: u32 = 0x58;
pub const kHIDUsage_BCS_LaserOnTime: u32 = 0x59;
pub const kHIDUsage_BCS_LaserState: u32 = 0x5A;
pub const kHIDUsage_BCS_LockoutTime: u32 = 0x5B;
pub const kHIDUsage_BCS_MotorState: u32 = 0x5C;
pub const kHIDUsage_BCS_MotorTimeout: u32 = 0x5D;
pub const kHIDUsage_BCS_PowerOnResetScanner: u32 = 0x5E;
pub const kHIDUsage_BCS_PreventReadOfBarcodes: u32 = 0x5F;
pub const kHIDUsage_BCS_InitiateBarcodeRead: u32 = 0x60;
pub const kHIDUsage_BCS_TriggerState: u32 = 0x61;
pub const kHIDUsage_BCS_TriggerMode: u32 = 0x62;
pub const kHIDUsage_BCS_TriggerModeBlinkingLaserOn: u32 = 0x63;
pub const kHIDUsage_BCS_TriggerModeContinuousLaserOn: u32 = 0x64;
pub const kHIDUsage_BCS_TriggerModeLaserOnWhilePulled: u32 = 0x65;
pub const kHIDUsage_BCS_TriggerModeLaserStaysOnAfterTriggerRelease: u32 = 0x66;
// Reserved 0x67 - 0x6C
pub const kHIDUsage_BCS_CommitParametersToNVM: u32 = 0x6D;
pub const kHIDUsage_BCS_ParameterScanning: u32 = 0x6E;
pub const kHIDUsage_BCS_ParametersChanged: u32 = 0x6F;
pub const kHIDUsage_BCS_SetParameterDefaultValues: u32 = 0x70;
// Reserved 0x71 - 0x74
pub const kHIDUsage_BCS_ScannerInCradle: u32 = 0x75;
pub const kHIDUsage_BCS_ScannerInRange: u32 = 0x76;
// Reserved 0x77 - 0x79
pub const kHIDUsage_BCS_AimDuration: u32 = 0x7A;
pub const kHIDUsage_BCS_GoodReadLampDuration: u32 = 0x7B;
pub const kHIDUsage_BCS_GoodReadLampIntensity: u32 = 0x7C;
pub const kHIDUsage_BCS_GoodReadLED: u32 = 0x7D;
pub const kHIDUsage_BCS_GoodReadToneFrequency: u32 = 0x7E;
pub const kHIDUsage_BCS_GoodReadToneLength: u32 = 0x7F;
pub const kHIDUsage_BCS_GoodReadToneVolume: u32 = 0x80;
// Reserved 0x81
pub const kHIDUsage_BCS_NoReadMessage: u32 = 0x82;
pub const kHIDUsage_BCS_NotOnFileVolume: u32 = 0x83;
pub const kHIDUsage_BCS_PowerupBeep: u32 = 0x84;
pub const kHIDUsage_BCS_SoundErrorBeep: u32 = 0x85;
pub const kHIDUsage_BCS_SoundGoodReadBeep: u32 = 0x86;
pub const kHIDUsage_BCS_SoundNotOnFileBeep: u32 = 0x87;
pub const kHIDUsage_BCS_GoodReadWhenToWrite: u32 = 0x88;
pub const kHIDUsage_BCS_GRWTIAfterDecode: u32 = 0x89;
pub const kHIDUsage_BCS_GRWTIBeep_LampAfterTransmit: u32 = 0x8A;
pub const kHIDUsage_BCS_GRWTINoBeep_LampUseAtAll: u32 = 0x8B;
// Reserved 0x8C - 0x90
pub const kHIDUsage_BCS_BooklandEAN: u32 = 0x91;
pub const kHIDUsage_BCS_ConvertEAN8To13Type: u32 = 0x92;
pub const kHIDUsage_BCS_ConvertUPCAToEAN_13: u32 = 0x93;
pub const kHIDUsage_BCS_ConvertUPC_EToA: u32 = 0x94;
pub const kHIDUsage_BCS_EAN_13: u32 = 0x95;
pub const kHIDUsage_BCS_EAN_8: u32 = 0x96;
pub const kHIDUsage_BCS_EAN_99_128_Mandatory: u32 = 0x97;
pub const kHIDUsage_BCS_EAN_99_P5_128_Optional: u32 = 0x98;
// Reserved 0x99
pub const kHIDUsage_BCS_UPC_EAN: u32 = 0x9A;
pub const kHIDUsage_BCS_UPC_EANCouponCode: u32 = 0x9B;
pub const kHIDUsage_BCS_UPC_EANPeriodicals: u32 = 0x9C;
pub const kHIDUsage_BCS_UPC_A: u32 = 0x9D;
pub const kHIDUsage_BCS_UPC_AWith128Mandatory: u32 = 0x9E;
pub const kHIDUsage_BCS_UPC_AWith128Optical: u32 = 0x9F;
pub const kHIDUsage_BCS_UPC_AWithP5Optional: u32 = 0xA0;
pub const kHIDUsage_BCS_UPC_E: u32 = 0xA1;
pub const kHIDUsage_BCS_UPC_E1: u32 = 0xA2;
// Reserved 0xA3 - 0xA8
pub const kHIDUsage_BCS_Periodical: u32 = 0xA9;
pub const kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus2: u32 = 0xAA;
pub const kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus2: u32 = 0xAB;
pub const kHIDUsage_BCS_PeriodicalIgnorePlus2: u32 = 0xAC;
pub const kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus5: u32 = 0xAD;
pub const kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus5: u32 = 0xAE;
pub const kHIDUsage_BCS_PeriodicalIgnorePlus5: u32 = 0xAF;
pub const kHIDUsage_BCS_Check: u32 = 0xB0;
pub const kHIDUsage_BCS_CheckDisablePrice: u32 = 0xB1;
pub const kHIDUsage_BCS_CheckEnable4DigitPrice: u32 = 0xB2;
pub const kHIDUsage_BCS_CheckEnable5DigitPrice: u32 = 0xB3;
pub const kHIDUsage_BCS_CheckEnableEuropean4DigitPrice: u32 = 0xB4;
pub const kHIDUsage_BCS_CheckEnableEuropean5DigitPrice: u32 = 0xB5;
// Reserved 0xB6
pub const kHIDUsage_BCS_EANTwoLabel: u32 = 0xB7;
pub const kHIDUsage_BCS_EANThreeLabel: u32 = 0xB8;
pub const kHIDUsage_BCS_EAN8FlagDigit1: u32 = 0xB9;
pub const kHIDUsage_BCS_EAN8FlagDigit2: u32 = 0xBA;
pub const kHIDUsage_BCS_EAN8FlagDigit3: u32 = 0xBB;
pub const kHIDUsage_BCS_EAN13FlagDigit1: u32 = 0xBC;
pub const kHIDUsage_BCS_EAN13FlagDigit2: u32 = 0xBD;
pub const kHIDUsage_BCS_EAN13FlagDigit3: u32 = 0xBE;
pub const kHIDUsage_BCS_AddEAN2_3LabelDefinition: u32 = 0xBF;
pub const kHIDUsage_BCS_ClearAllEAN2_3LabelDefinitions: u32 = 0xC0;
// Reserved 0xC1 - 0xC2
pub const kHIDUsage_BCS_Codabar: u32 = 0xC3;
pub const kHIDUsage_BCS_Code128: u32 = 0xC4;
// Reserved 0xC5 - 0xC6
pub const kHIDUsage_BCS_Code39: u32 = 0xC7;
pub const kHIDUsage_BCS_Code93: u32 = 0xC8;
pub const kHIDUsage_BCS_FullASCIIConversion: u32 = 0xC9;
pub const kHIDUsage_BCS_Interleaved2of5: u32 = 0xCA;
pub const kHIDUsage_BCS_ItalianPharmacyCode: u32 = 0xCB;
pub const kHIDUsage_BCS_MSI_Plessey: u32 = 0xCC;
pub const kHIDUsage_BCS_Standard2of5IATA: u32 = 0xCD;
pub const kHIDUsage_BCS_Standard2of5: u32 = 0xCE;
// Reserved 0xCF - 0xD2
pub const kHIDUsage_BCS_TransmitStart_Stop: u32 = 0xD3;
pub const kHIDUsage_BCS_TriOptic: u32 = 0xD4;
pub const kHIDUsage_BCS_UCC_EAN_128: u32 = 0xD5;
pub const kHIDUsage_BCS_CheckDigit: u32 = 0xD6;
pub const kHIDUsage_BCS_CheckDigitDisable: u32 = 0xD7;
pub const kHIDUsage_BCS_CheckDigitEnableInterleaved2of5OPCC: u32 = 0xD8;
pub const kHIDUsage_BCS_CheckDigitEnableInterleaved2of5USS: u32 = 0xD9;
pub const kHIDUsage_BCS_CheckDigitEnableStandard2of5OPCC: u32 = 0xD8;
pub const kHIDUsage_BCS_CheckDigitEnableStandard2of5USS: u32 = 0xD9;
pub const kHIDUsage_BCS_CheckDigitEnableOneMSIPlessey: u32 = 0xDC;
pub const kHIDUsage_BCS_CheckDigitEnableTwoMSIPlessey: u32 = 0xDD;
pub const kHIDUsage_BCS_CheckDigitCodabarEnable: u32 = 0xDE;
pub const kHIDUsage_BCS_CheckDigitCode99Enable: u32 = 0xDF;
// Reserved 0xE0 - 0xEF
pub const kHIDUsage_BCS_TransmitCheckDigit: u32 = 0xF0;
pub const kHIDUsage_BCS_DisableCheckDigitTransmit: u32 = 0xF1;
pub const kHIDUsage_BCS_EnableCheckDigitTransmit: u32 = 0xF2;
// Reserved 0xF3 - 0xFA
pub const kHIDUsage_BCS_SymbologyIdentifier1: u32 = 0xFB;
pub const kHIDUsage_BCS_SymbologyIdentifier2: u32 = 0xFC;
pub const kHIDUsage_BCS_SymbologyIdentifier3: u32 = 0xFD;
pub const kHIDUsage_BCS_DecodedData: u32 = 0xFE;
pub const kHIDUsage_BCS_DecodeDataContinued: u32 = 0xFF;
pub const kHIDUsage_BCS_BarSpaceData: u32 = 0x100;
pub const kHIDUsage_BCS_ScannerDataAccuracy: u32 = 0x101;
pub const kHIDUsage_BCS_RawDataPolarity: u32 = 0x102;
pub const kHIDUsage_BCS_PolarityInvertedBarCode: u32 = 0x103;
pub const kHIDUsage_BCS_PolarityNormalBarCode: u32 = 0x103;
// Reserved 0x105
pub const kHIDUsage_BCS_MinimumLengthToDecode: u32 = 0x106;
pub const kHIDUsage_BCS_MaximumLengthToDecode: u32 = 0x107;
pub const kHIDUsage_BCS_FirstDiscreteLengthToDecode: u32 = 0x108;
pub const kHIDUsage_BCS_SecondDiscreteLengthToDecode: u32 = 0x109;
pub const kHIDUsage_BCS_DataLengthMethod: u32 = 0x10A;
pub const kHIDUsage_BCS_DLMethodReadAny: u32 = 0x10B;
pub const kHIDUsage_BCS_DLMethodCheckInRange: u32 = 0x10C;
pub const kHIDUsage_BCS_DLMethodCheckForDiscrete: u32 = 0x10D;
// Reserved 0x10E - 0x10F
pub const kHIDUsage_BCS_AztecCode: u32 = 0x110;
pub const kHIDUsage_BCS_BC412: u32 = 0x111;
pub const kHIDUsage_BCS_ChannelCode: u32 = 0x112;
pub const kHIDUsage_BCS_Code16: u32 = 0x113;
pub const kHIDUsage_BCS_Code32: u32 = 0x114;
pub const kHIDUsage_BCS_Code49: u32 = 0x115;
pub const kHIDUsage_BCS_CodeOne: u32 = 0x116;
pub const kHIDUsage_BCS_Colorcode: u32 = 0x117;
pub const kHIDUsage_BCS_DataMatrix: u32 = 0x118;
pub const kHIDUsage_BCS_MaxiCode: u32 = 0x119;
pub const kHIDUsage_BCS_MicroPDF: u32 = 0x11A;
pub const kHIDUsage_BCS_PDF_417: u32 = 0x11B;
pub const kHIDUsage_BCS_PosiCode: u32 = 0x11C;
pub const kHIDUsage_BCS_QRCode: u32 = 0x11D;
pub const kHIDUsage_BCS_SuperCode: u32 = 0x11E;
pub const kHIDUsage_BCS_UltraCode: u32 = 0x11F;
pub const kHIDUsage_BCS_USB_5_SlugCode: u32 = 0x120;
pub const kHIDUsage_BCS_VeriCode: u32 = 0x121;
// Reserved 0x122 - 0xFFFF

// Weighing Devices Page (0x8D)
pub const kHIDUsage_WD_Undefined: u32 = 0x00;
pub const kHIDUsage_WD_WeighingDevice: u32 = 0x01;
// Reserved 0x02 - 0x1F
pub const kHIDUsage_WD_ScaleScaleDevice: u32 = 0x20;
pub const kHIDUsage_WD_ScaleScaleClassIMetricCL: u32 = 0x21;
pub const kHIDUsage_WD_ScaleScaleClassIMetric: u32 = 0x22;
pub const kHIDUsage_WD_ScaleScaleClassIIMetric: u32 = 0x23;
pub const kHIDUsage_WD_ScaleScaleClassIIIMetric: u32 = 0x24;
pub const kHIDUsage_WD_ScaleScaleClassIIILMetric: u32 = 0x25;
pub const kHIDUsage_WD_ScaleScaleClassIVMetric: u32 = 0x26;
pub const kHIDUsage_WD_ScaleScaleClassIIIEnglish: u32 = 0x27;
pub const kHIDUsage_WD_ScaleScaleClassIIILEnglish: u32 = 0x28;
pub const kHIDUsage_WD_ScaleScaleClassIVEnglish: u32 = 0x29;
pub const kHIDUsage_WD_ScaleScaleClassGeneric: u32 = 0x2A;
// Reserved 0x2B - 0x2F
pub const kHIDUsage_WD_ScaleAtrributeReport: u32 = 0x30;
pub const kHIDUsage_WD_ScaleControlReport: u32 = 0x31;
pub const kHIDUsage_WD_ScaleDataReport: u32 = 0x32;
pub const kHIDUsage_WD_ScaleStatusReport: u32 = 0x33;
pub const kHIDUsage_WD_ScaleWeightLimitReport: u32 = 0x34;
pub const kHIDUsage_WD_ScaleStatisticsReport: u32 = 0x35;
// Reserved 0x36 - 0x3F
pub const kHIDUsage_WD_DataWeight: u32 = 0x40;
pub const kHIDUsage_WD_DataScaling: u32 = 0x41;
// Reserved 0x42 - 0x4F
pub const kHIDUsage_WD_WeightUnit: u32 = 0x50;
pub const kHIDUsage_WD_WeightUnitMilligram: u32 = 0x51;
pub const kHIDUsage_WD_WeightUnitGram: u32 = 0x52;
pub const kHIDUsage_WD_WeightUnitKilogram: u32 = 0x53;
pub const kHIDUsage_WD_WeightUnitCarats: u32 = 0x54;
pub const kHIDUsage_WD_WeightUnitTaels: u32 = 0x55;
pub const kHIDUsage_WD_WeightUnitGrains: u32 = 0x56;
pub const kHIDUsage_WD_WeightUnitPennyweights: u32 = 0x57;
pub const kHIDUsage_WD_WeightUnitMetricTon: u32 = 0x58;
pub const kHIDUsage_WD_WeightUnitAvoirTon: u32 = 0x59;
pub const kHIDUsage_WD_WeightUnitTroyOunce: u32 = 0x5A;
pub const kHIDUsage_WD_WeightUnitOunce: u32 = 0x5B;
pub const kHIDUsage_WD_WeightUnitPound: u32 = 0x5C;
// Reserved 0x5D - 0x5F
pub const kHIDUsage_WD_CalibrationCount: u32 = 0x60;
pub const kHIDUsage_WD_RezeroCount: u32 = 0x61;
// Reserved 0x62 - 0x6F
pub const kHIDUsage_WD_ScaleStatus: u32 = 0x70;
pub const kHIDUsage_WD_ScaleStatusFault: u32 = 0x71;
pub const kHIDUsage_WD_ScaleStatusStableAtZero: u32 = 0x72;
pub const kHIDUsage_WD_ScaleStatusInMotion: u32 = 0x73;
pub const kHIDUsage_WD_ScaleStatusWeightStable: u32 = 0x74;
pub const kHIDUsage_WD_ScaleStatusUnderZero: u32 = 0x75;
pub const kHIDUsage_WD_ScaleStatusOverWeightLimit: u32 = 0x76;
pub const kHIDUsage_WD_ScaleStatusRequiresCalibration: u32 = 0x77;
pub const kHIDUsage_WD_ScaleStatusRequiresRezeroing: u32 = 0x78;
// Reserved 0x79 - 0x7F
pub const kHIDUsage_WD_ZeroScale: u32 = 0x80;
pub const kHIDUsage_WD_EnforcedZeroReturn: u32 = 0x81;
// Reserved 0x82 - 0xFFFF

// Magnetic Stripe Reader Page (0x8E)
pub const kHIDUsage_MSR_Undefined: u32 = 0x00;
pub const kHIDUsage_MSR_DeviceReadOnly: u32 = 0x01;
// Reserved 0x02 - 0x10
pub const kHIDUsage_MSR_Track1Length: u32 = 0x11;
pub const kHIDUsage_MSR_Track2Length: u32 = 0x12;
pub const kHIDUsage_MSR_Track3Length: u32 = 0x13;
pub const kHIDUsage_MSR_TrackJISLength: u32 = 0x14;
// Reserved 0x15 - 0x1F
pub const kHIDUsage_MSR_TrackData: u32 = 0x20;
pub const kHIDUsage_MSR_Track1Data: u32 = 0x21;
pub const kHIDUsage_MSR_Track2Data: u32 = 0x22;
pub const kHIDUsage_MSR_Track3Data: u32 = 0x23;
pub const kHIDUsage_MSR_TrackJISData: u32 = 0x24;
// Reserved 0x25 - 0xFFFF
