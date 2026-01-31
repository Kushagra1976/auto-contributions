#!/bin/bash
#
# Learning Objective: This tutorial will teach you how to work with files and directories in Bash.
# We will cover creating, listing, moving, copying, and deleting files and directories.
# This is fundamental for managing your system and automating tasks.

# --- Introduction ---
# Bash provides powerful commands for interacting with your file system.
# Understanding these commands will make you more efficient when working on the command line.

# --- Creating Files and Directories ---

# Creating an empty file using the 'touch' command.
# 'touch' creates a new file if it doesn't exist, or updates its timestamp if it does.
echo "Creating an empty file named 'my_new_file.txt'..."
touch my_new_file.txt

# Creating a file with content using redirection.
# The '>' operator redirects the output of the 'echo' command into the specified file.
# If the file exists, it will be overwritten.
echo "Creating a file named 'another_file.txt' with some content..."
echo "This is the first line of text." > another_file.txt
echo "This is the second line." >> another_file.txt # '>>' appends to the file, doesn't overwrite.

# Creating a new directory using the 'mkdir' command.
# 'mkdir' stands for "make directory".
echo "Creating a directory named 'my_new_directory'..."
mkdir my_new_directory

# Creating nested directories (multiple levels) at once.
# The '-p' option (parents) allows creating parent directories as needed.
echo "Creating nested directories 'parent_dir/child_dir/grandchild_dir'..."
mkdir -p parent_dir/child_dir/grandchild_dir

# --- Listing Files and Directories ---

# Listing the contents of the current directory.
# 'ls' is the standard command for listing directory contents.
echo "Listing contents of the current directory:"
ls

# Listing with more details (long format).
# The '-l' option provides detailed information like permissions, owner, size, and modification date.
echo "Listing with details:"
ls -l

# Listing all files, including hidden ones (those starting with a dot).
# The '-a' option shows all entries, including '.' (current directory) and '..' (parent directory).
echo "Listing all files (including hidden):"
ls -a

# --- Moving and Renaming Files and Directories ---

# Moving a file.
# The 'mv' command is used for both moving and renaming.
# To move, specify the source file and the destination directory.
echo "Moving 'my_new_file.txt' into 'my_new_directory'..."
mv my_new_file.txt my_new_directory/

# Renaming a file.
# To rename, specify the old name and the new name in the same directory.
echo "Renaming 'another_file.txt' to 'renamed_file.txt'..."
mv another_file.txt renamed_file.txt

# Moving and renaming a directory.
echo "Moving 'parent_dir' into 'my_new_directory'..."
mv parent_dir my_new_directory/

# --- Copying Files and Directories ---

# Copying a file.
# The 'cp' command is used for copying files.
# Specify the source file and the destination.
echo "Copying 'renamed_file.txt' into 'my_new_directory'..."
cp renamed_file.txt my_new_directory/

# Copying a directory recursively.
# The '-r' option (recursive) is necessary to copy directories and their contents.
# Let's create a sample directory to copy.
echo "Creating a sample directory for recursive copy..."
mkdir sample_dir
echo "Some content" > sample_dir/file_in_sample.txt
echo "Copying 'sample_dir' into 'my_new_directory' recursively..."
cp -r sample_dir my_new_directory/

# --- Deleting Files and Directories ---

# Deleting a file.
# The 'rm' command is used for removing (deleting) files.
echo "Deleting the file 'renamed_file.txt'..."
rm renamed_file.txt

# Deleting an empty directory.
# The 'rmdir' command is used for removing empty directories.
echo "Attempting to remove the empty directory 'my_new_directory/child_dir' (it's not empty, will fail)..."
rmdir my_new_directory/child_dir # This will likely fail as it's not empty.

# Deleting a directory and its contents recursively.
# Use 'rm -r' with caution! This permanently deletes files and directories without confirmation by default.
# The '-f' option forces deletion without prompting (use with extreme care).
echo "Deleting the directory 'my_new_directory' and all its contents..."
rm -rf my_new_directory

# --- Example Usage ---
echo ""
echo "--- Example: Creating a project structure ---"
echo "Let's create a simple project directory with subfolders and files."

# Create a main project directory
mkdir my_project
cd my_project # Change directory into our new project

# Create source and documentation subdirectories
mkdir src docs
cd src

# Create a main script file and a helper file
echo "#!/bin/bash" > main.sh
echo "echo 'Hello from main!'" >> main.sh
chmod +x main.sh # Make the script executable

echo "# Utility functions" > utils.sh
echo "my_function() { echo 'This is a utility function.'; }" >> utils.sh

# Go back to the project root and create a README in docs
cd ../docs
echo "Project README" > README.md
echo "This is a sample project." >> README.md

# Go back to the root of the current script's directory
cd ..

echo "Project structure created in 'my_project'."
echo "Listing the contents of 'my_project':"
ls -R # '-R' for recursive listing of contents

# Clean up the example
echo ""
echo "Cleaning up the 'my_project' directory..."
cd .. # Move out of my_project before deleting it
rm -rf my_project
echo "Cleanup complete."
echo ""
echo "Congratulations! You've learned the basics of file and directory management in Bash."
echo "Experiment with these commands to become more comfortable."