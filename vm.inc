%define shm0 0
%define shm1 1
%define shm2 2
%define shm3 3
%define shm4 4
%define shm5 5
%define shm6 6
%define shm7 7


%macro shmend 0
db 0x00
%endmacro

%macro shmovi 2
  db 0x1, %1
  dw %2
%endmacro

%macro shmov 2
  db 0x2, %1, %2
%endmacro

%macro shmadd 2
  db 0x3, %1
  dw %2
%endmacro

%macro shmsub 2
  db 0x4, %1
  dw %2
%endmacro

%macro shmand 2
  db 0x5, %1, %2
%endmacro

%macro shmor 2
  db 0x6, %1, %2
%endmacro

%macro shmxor 2
  db 0x7, %1, %2
%endmacro