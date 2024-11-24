from pathlib import Path
import shutil
import subprocess

def prepare_tempdir():
    temp_dir = Path(".tmp")
    if not temp_dir.exists():
        temp_dir.mkdir()
    else:
        shutil.rmtree(temp_dir)
        temp_dir.mkdir()
    return temp_dir

def test_generate_template():
    temp_dir = prepare_tempdir()
    subprocess.run([
        "cargo","generate",
        "-p","../template",
        "-n","test-demo",
    ],cwd=temp_dir)
if __name__ == '__main__':
    test_generate_template()
