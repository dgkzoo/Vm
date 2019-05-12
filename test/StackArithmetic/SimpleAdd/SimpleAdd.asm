@7    // ***push constant 7 ***
D=A    // D=A(constant 7)
@SP    // Areg=0x00
A=M    // A=M[SP]
M=D    // push (7) // M[SP]=D(constant 7)
@SP    // Areg=0x00
M=M+1  // SP inc // M[SP]=M[SP]+1
@8    // ***push constant 8 ***
D=A    // D=A(constant 8)
@SP    // Areg=0x00
A=M    // A=M[SP]
M=D    // push (8) // M[SP]=D(constant 8)
@SP    // Areg=0x00
M=M+1  // SP inc // M[SP]=M[SP]+1
@SP   // ***add***
M=M-1 // SP - 1
A=M   // A=M[SP](SP Address)
D=M   // D=M(val2 to D)
@SP   // Areg=0
M=M-1 // SP-1
A=M   // A=M[SP](SP Address)
M=D+M // add
@SP   // Areg=0
M=M+1 // SP + 1
