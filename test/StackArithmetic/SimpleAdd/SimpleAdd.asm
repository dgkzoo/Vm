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
@0    // ***push local 0 ***
D=A    // D=0)
@LCL   // Areg=300
D=D+M  // D=local address(LCL+index)
A=D    // A=local address(LCL+index)
D=M    // D=local val
@SP    // Areg=0
A=M    // A=M[SP]
M=D    // push
@SP   // Areg=0
M=M+1  // SP inc // M[SP]=M[SP]+1
@1    // ***push local 1 ***
D=A    // D=1)
@LCL   // Areg=300
D=D+M  // D=local address(LCL+index)
A=D    // A=local address(LCL+index)
D=M    // D=local val
@SP    // Areg=0
A=M    // A=M[SP]
M=D    // push
@SP   // Areg=0
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
