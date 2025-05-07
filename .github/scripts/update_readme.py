import sys

def update_readme(help_file, readme_file, line_number):
    with open(help_file, 'r') as help_f:
        help_output = help_f.read()

    with open(readme_file, 'r') as readme_f:
        lines = readme_f.readlines()

    # Insert the help output at the specified line
    lines.insert(line_number - 1, "```\n" + help_output + "```\n")

    with open(readme_file, 'w') as readme_f:
        readme_f.writelines(lines)

if __name__ == "__main__":
    help_file = sys.argv[1]
    readme_file = sys.argv[2]
    line_number = int(sys.argv[3])
    update_readme(help_file, readme_file, line_number)