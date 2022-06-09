Pentru a testa este necesar compilatorul rust https://www.rust-lang.org/tools/install

Dupa instalarea acestuia, se deschide un terminal in folderul proiectului si se ruleaza:
```
cargo run
```
Programul isi va incepe executia si putem incepe testarea. Un exemplu de secventa de test:
```
ADC $1B 100
ADC $1C 150
ADC $1D 50
```

Perifericul cu LED RGB va folosi mereu aceste adrese pentru a-si lua valorile necesare

Instructiuni implementate:
``` 
ADC
CLC
CLD
CLI
CLV
DEC
DEX
DEY
INC
LDA
LDX
LDY
```