#![allow(non_upper_case_globals)]


pub type SDLKey = cty::c_uint;

pub const SDLK_UNKNOWN: SDLKey = 0;
pub const SDLK_BACKSPACE: SDLKey = 8;
pub const SDLK_TAB: SDLKey = 9;
pub const SDLK_CLEAR: SDLKey = 12;
pub const SDLK_RETURN: SDLKey = 13;
pub const SDLK_PAUSE: SDLKey = 19;
pub const SDLK_ESCAPE: SDLKey = 27;
pub const SDLK_SPACE: SDLKey = 32;
pub const SDLK_EXCLAIM: SDLKey = 33;
pub const SDLK_QUOTEDBL: SDLKey = 34;
pub const SDLK_HASH: SDLKey = 35;
pub const SDLK_DOLLAR: SDLKey = 36;
pub const SDLK_AMPERSAND: SDLKey = 38;
pub const SDLK_QUOTE: SDLKey = 39;
pub const SDLK_LEFTPAREN: SDLKey = 40;
pub const SDLK_RIGHTPAREN: SDLKey = 41;
pub const SDLK_ASTERISK: SDLKey = 42;
pub const SDLK_PLUS: SDLKey = 43;
pub const SDLK_COMMA: SDLKey = 44;
pub const SDLK_MINUS: SDLKey = 45;
pub const SDLK_PERIOD: SDLKey = 46;
pub const SDLK_SLASH: SDLKey = 47;
pub const SDLK_0: SDLKey = 48;
pub const SDLK_1: SDLKey = 49;
pub const SDLK_2: SDLKey = 50;
pub const SDLK_3: SDLKey = 51;
pub const SDLK_4: SDLKey = 52;
pub const SDLK_5: SDLKey = 53;
pub const SDLK_6: SDLKey = 54;
pub const SDLK_7: SDLKey = 55;
pub const SDLK_8: SDLKey = 56;
pub const SDLK_9: SDLKey = 57;
pub const SDLK_COLON: SDLKey = 58;
pub const SDLK_SEMICOLON: SDLKey = 59;
pub const SDLK_LESS: SDLKey = 60;
pub const SDLK_EQUALS: SDLKey = 61;
pub const SDLK_GREATER: SDLKey = 62;
pub const SDLK_QUESTION: SDLKey = 63;
pub const SDLK_AT: SDLKey = 64;
pub const SDLK_LEFTBRACKET: SDLKey = 91;
pub const SDLK_BACKSLASH: SDLKey = 92;
pub const SDLK_RIGHTBRACKET: SDLKey = 93;
pub const SDLK_CARET: SDLKey = 94;
pub const SDLK_UNDERSCORE: SDLKey = 95;
pub const SDLK_BACKQUOTE: SDLKey = 96;
pub const SDLK_a: SDLKey = 97;
pub const SDLK_b: SDLKey = 98;
pub const SDLK_c: SDLKey = 99;
pub const SDLK_d: SDLKey = 100;
pub const SDLK_e: SDLKey = 101;
pub const SDLK_f: SDLKey = 102;
pub const SDLK_g: SDLKey = 103;
pub const SDLK_h: SDLKey = 104;
pub const SDLK_i: SDLKey = 105;
pub const SDLK_j: SDLKey = 106;
pub const SDLK_k: SDLKey = 107;
pub const SDLK_l: SDLKey = 108;
pub const SDLK_m: SDLKey = 109;
pub const SDLK_n: SDLKey = 110;
pub const SDLK_o: SDLKey = 111;
pub const SDLK_p: SDLKey = 112;
pub const SDLK_q: SDLKey = 113;
pub const SDLK_r: SDLKey = 114;
pub const SDLK_s: SDLKey = 115;
pub const SDLK_t: SDLKey = 116;
pub const SDLK_u: SDLKey = 117;
pub const SDLK_v: SDLKey = 118;
pub const SDLK_w: SDLKey = 119;
pub const SDLK_x: SDLKey = 120;
pub const SDLK_y: SDLKey = 121;
pub const SDLK_z: SDLKey = 122;
pub const SDLK_DELETE: SDLKey = 127;
pub const SDLK_WORLD_0: SDLKey = 160;
pub const SDLK_WORLD_1: SDLKey = 161;
pub const SDLK_WORLD_2: SDLKey = 162;
pub const SDLK_WORLD_3: SDLKey = 163;
pub const SDLK_WORLD_4: SDLKey = 164;
pub const SDLK_WORLD_5: SDLKey = 165;
pub const SDLK_WORLD_6: SDLKey = 166;
pub const SDLK_WORLD_7: SDLKey = 167;
pub const SDLK_WORLD_8: SDLKey = 168;
pub const SDLK_WORLD_9: SDLKey = 169;
pub const SDLK_WORLD_10: SDLKey = 170;
pub const SDLK_WORLD_11: SDLKey = 171;
pub const SDLK_WORLD_12: SDLKey = 172;
pub const SDLK_WORLD_13: SDLKey = 173;
pub const SDLK_WORLD_14: SDLKey = 174;
pub const SDLK_WORLD_15: SDLKey = 175;
pub const SDLK_WORLD_16: SDLKey = 176;
pub const SDLK_WORLD_17: SDLKey = 177;
pub const SDLK_WORLD_18: SDLKey = 178;
pub const SDLK_WORLD_19: SDLKey = 179;
pub const SDLK_WORLD_20: SDLKey = 180;
pub const SDLK_WORLD_21: SDLKey = 181;
pub const SDLK_WORLD_22: SDLKey = 182;
pub const SDLK_WORLD_23: SDLKey = 183;
pub const SDLK_WORLD_24: SDLKey = 184;
pub const SDLK_WORLD_25: SDLKey = 185;
pub const SDLK_WORLD_26: SDLKey = 186;
pub const SDLK_WORLD_27: SDLKey = 187;
pub const SDLK_WORLD_28: SDLKey = 188;
pub const SDLK_WORLD_29: SDLKey = 189;
pub const SDLK_WORLD_30: SDLKey = 190;
pub const SDLK_WORLD_31: SDLKey = 191;
pub const SDLK_WORLD_32: SDLKey = 192;
pub const SDLK_WORLD_33: SDLKey = 193;
pub const SDLK_WORLD_34: SDLKey = 194;
pub const SDLK_WORLD_35: SDLKey = 195;
pub const SDLK_WORLD_36: SDLKey = 196;
pub const SDLK_WORLD_37: SDLKey = 197;
pub const SDLK_WORLD_38: SDLKey = 198;
pub const SDLK_WORLD_39: SDLKey = 199;
pub const SDLK_WORLD_40: SDLKey = 200;
pub const SDLK_WORLD_41: SDLKey = 201;
pub const SDLK_WORLD_42: SDLKey = 202;
pub const SDLK_WORLD_43: SDLKey = 203;
pub const SDLK_WORLD_44: SDLKey = 204;
pub const SDLK_WORLD_45: SDLKey = 205;
pub const SDLK_WORLD_46: SDLKey = 206;
pub const SDLK_WORLD_47: SDLKey = 207;
pub const SDLK_WORLD_48: SDLKey = 208;
pub const SDLK_WORLD_49: SDLKey = 209;
pub const SDLK_WORLD_50: SDLKey = 210;
pub const SDLK_WORLD_51: SDLKey = 211;
pub const SDLK_WORLD_52: SDLKey = 212;
pub const SDLK_WORLD_53: SDLKey = 213;
pub const SDLK_WORLD_54: SDLKey = 214;
pub const SDLK_WORLD_55: SDLKey = 215;
pub const SDLK_WORLD_56: SDLKey = 216;
pub const SDLK_WORLD_57: SDLKey = 217;
pub const SDLK_WORLD_58: SDLKey = 218;
pub const SDLK_WORLD_59: SDLKey = 219;
pub const SDLK_WORLD_60: SDLKey = 220;
pub const SDLK_WORLD_61: SDLKey = 221;
pub const SDLK_WORLD_62: SDLKey = 222;
pub const SDLK_WORLD_63: SDLKey = 223;
pub const SDLK_WORLD_64: SDLKey = 224;
pub const SDLK_WORLD_65: SDLKey = 225;
pub const SDLK_WORLD_66: SDLKey = 226;
pub const SDLK_WORLD_67: SDLKey = 227;
pub const SDLK_WORLD_68: SDLKey = 228;
pub const SDLK_WORLD_69: SDLKey = 229;
pub const SDLK_WORLD_70: SDLKey = 230;
pub const SDLK_WORLD_71: SDLKey = 231;
pub const SDLK_WORLD_72: SDLKey = 232;
pub const SDLK_WORLD_73: SDLKey = 233;
pub const SDLK_WORLD_74: SDLKey = 234;
pub const SDLK_WORLD_75: SDLKey = 235;
pub const SDLK_WORLD_76: SDLKey = 236;
pub const SDLK_WORLD_77: SDLKey = 237;
pub const SDLK_WORLD_78: SDLKey = 238;
pub const SDLK_WORLD_79: SDLKey = 239;
pub const SDLK_WORLD_80: SDLKey = 240;
pub const SDLK_WORLD_81: SDLKey = 241;
pub const SDLK_WORLD_82: SDLKey = 242;
pub const SDLK_WORLD_83: SDLKey = 243;
pub const SDLK_WORLD_84: SDLKey = 244;
pub const SDLK_WORLD_85: SDLKey = 245;
pub const SDLK_WORLD_86: SDLKey = 246;
pub const SDLK_WORLD_87: SDLKey = 247;
pub const SDLK_WORLD_88: SDLKey = 248;
pub const SDLK_WORLD_89: SDLKey = 249;
pub const SDLK_WORLD_90: SDLKey = 250;
pub const SDLK_WORLD_91: SDLKey = 251;
pub const SDLK_WORLD_92: SDLKey = 252;
pub const SDLK_WORLD_93: SDLKey = 253;
pub const SDLK_WORLD_94: SDLKey = 254;
pub const SDLK_WORLD_95: SDLKey = 255;
pub const SDLK_KP0: SDLKey = 256;
pub const SDLK_KP1: SDLKey = 257;
pub const SDLK_KP2: SDLKey = 258;
pub const SDLK_KP3: SDLKey = 259;
pub const SDLK_KP4: SDLKey = 260;
pub const SDLK_KP5: SDLKey = 261;
pub const SDLK_KP6: SDLKey = 262;
pub const SDLK_KP7: SDLKey = 263;
pub const SDLK_KP8: SDLKey = 264;
pub const SDLK_KP9: SDLKey = 265;
pub const SDLK_KP_PERIOD: SDLKey = 266;
pub const SDLK_KP_DIVIDE: SDLKey = 267;
pub const SDLK_KP_MULTIPLY: SDLKey = 268;
pub const SDLK_KP_MINUS: SDLKey = 269;
pub const SDLK_KP_PLUS: SDLKey = 270;
pub const SDLK_KP_ENTER: SDLKey = 271;
pub const SDLK_KP_EQUALS: SDLKey = 272;
pub const SDLK_UP: SDLKey = 273;
pub const SDLK_DOWN: SDLKey = 274;
pub const SDLK_RIGHT: SDLKey = 275;
pub const SDLK_LEFT: SDLKey = 276;
pub const SDLK_INSERT: SDLKey = 277;
pub const SDLK_HOME: SDLKey = 278;
pub const SDLK_END: SDLKey = 279;
pub const SDLK_PAGEUP: SDLKey = 280;
pub const SDLK_PAGEDOWN: SDLKey = 281;
pub const SDLK_F1: SDLKey = 282;
pub const SDLK_F2: SDLKey = 283;
pub const SDLK_F3: SDLKey = 284;
pub const SDLK_F4: SDLKey = 285;
pub const SDLK_F5: SDLKey = 286;
pub const SDLK_F6: SDLKey = 287;
pub const SDLK_F7: SDLKey = 288;
pub const SDLK_F8: SDLKey = 289;
pub const SDLK_F9: SDLKey = 290;
pub const SDLK_F10: SDLKey = 291;
pub const SDLK_F11: SDLKey = 292;
pub const SDLK_F12: SDLKey = 293;
pub const SDLK_F13: SDLKey = 294;
pub const SDLK_F14: SDLKey = 295;
pub const SDLK_F15: SDLKey = 296;
pub const SDLK_NUMLOCK: SDLKey = 300;
pub const SDLK_CAPSLOCK: SDLKey = 301;
pub const SDLK_SCROLLOCK: SDLKey = 302;
pub const SDLK_RSHIFT: SDLKey = 303;
pub const SDLK_LSHIFT: SDLKey = 304;
pub const SDLK_RCTRL: SDLKey = 305;
pub const SDLK_LCTRL: SDLKey = 306;
pub const SDLK_RALT: SDLKey = 307;
pub const SDLK_LALT: SDLKey = 308;
pub const SDLK_RMETA: SDLKey = 309;
pub const SDLK_LMETA: SDLKey = 310;
pub const SDLK_LSUPER: SDLKey = 311;
pub const SDLK_RSUPER: SDLKey = 312;
pub const SDLK_MODE: SDLKey = 313;
pub const SDLK_COMPOSE: SDLKey = 314;
pub const SDLK_HELP: SDLKey = 315;
pub const SDLK_PRINT: SDLKey = 316;
pub const SDLK_SYSREQ: SDLKey = 317;
pub const SDLK_BREAK: SDLKey = 318;
pub const SDLK_MENU: SDLKey = 319;
pub const SDLK_POWER: SDLKey = 320;
pub const SDLK_EURO: SDLKey = 321;
pub const SDLK_UNDO: SDLKey = 322;
