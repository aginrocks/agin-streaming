main:
	cmake -S . -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
	echo "done compiling"
	echo "creating compile_commands.json"
	ln -s build/compile_commands.json compile_commands.json
clean:
	unlink compile_commands.json
