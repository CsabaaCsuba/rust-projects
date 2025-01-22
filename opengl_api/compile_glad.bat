gcc -c -Iglad/include -DBUILD_DLL glad/src/glad.c -o glad/src/glad.o
gcc glad/src/glad.o -shared -lgdi32 -lshell32 -o glad/src/glad.dll