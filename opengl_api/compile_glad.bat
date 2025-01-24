gcc -c -Iglad/include -DBUILD_DLL glad/src/glad.c -o glad/src/glad.o
gcc glad/src/glad.o -shared  -lshell32 -lopengl32 -lglu32 -lgdi32 -o glad/src/glad.dll