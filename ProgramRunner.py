import subprocess
import time
import os
import signal

# Path to your .exe file
exe_path = r"C:\Downloads\Runner.exe"

# Start the process
process = subprocess.Popen(exe_path)

# Wait for 1 second
time.sleep(1)

# Terminate the process
if os.name == 'nt':
    # For Windows
    subprocess.call(['taskkill', '/F', '/T', '/PID', str(process.pid)])
else:
    # For Unix-based systems
    os.kill(process.pid, signal.SIGTERM)
