add:
	push rbp
	mov rbp, rsp
	mov edi, dword ptr [rbp-20]
	mov esi, dword ptr [rbp-24]
	mov eax, 5
	pop rbp
	ret

global _start
_start:
	push rbp
	mov rbp, rsp
	mov rax, 60
	mov rdi, 1
	pop rbp
	syscall
