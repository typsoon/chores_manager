import os

# List of files in the required order with '../' added before 'main files'
my_input_files = [
    "./createTables.sql",
    "./createPrivateViews.sql",
    "./createFunctions.sql",
    "./createViews.sql",
    "./createRules.sql",
    "./createTriggers.sql",
]

my_output_file = '../migrations/create_structure/up.sql'

def concatenate_files_with_titles(input_files, output_file):
    with open(output_file, 'w') as outfile:
        for input_file in input_files:
            # Write the title (filename) to the output file
            title = os.path.basename(input_file)
            outfile.write(f"-- {title}\n")
            outfile.write("--" + "=" * len(title) + "\n\n")

            # Write the content of the input file to the output file
            with open(input_file, 'r') as infile:
                outfile.write(infile.read())

            # Add a separator between files (optional)
            outfile.write("\n\n")


concatenate_files_with_titles(my_input_files, my_output_file)

print(f"Files have been concatenated into {my_output_file}")

