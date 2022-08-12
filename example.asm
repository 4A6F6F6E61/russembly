
jmp init
ORG 003h
cpl P2.2
clr IEO
reti

init:
setb IT0
setb EX0
setb EA
mov P1, #0
mov P2, #0

main:
    jnb P1.1, leon
    setb P2.1
    jmp main
leon:
    clr P2.1
    jmp main

jmp main
end
