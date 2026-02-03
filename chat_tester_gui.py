#!/usr/bin/env python3
import tkinter as tk
from tkinter import scrolledtext, ttk
import subprocess
import threading
import time
import pty
import os
import select

class OCLITester:
    def __init__(self, root):
        self.root = root
        root.title("OCLI Conversational Tester")
        root.geometry("900x700")
        
        # Output area
        tk.Label(root, text="OCLI Output:", font=("Courier", 12, "bold")).pack(pady=5)
        self.output = scrolledtext.ScrolledText(root, height=25, width=100, font=("Courier", 10))
        self.output.pack(padx=10, pady=5)
        
        # Input area
        tk.Label(root, text="Send to OCLI:", font=("Courier", 12, "bold")).pack(pady=5)
        self.input_frame = tk.Frame(root)
        self.input_frame.pack(padx=10, pady=5, fill=tk.X)
        
        self.input_box = tk.Entry(self.input_frame, font=("Courier", 11))
        self.input_box.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 5))
        self.input_box.bind("<Return>", lambda e: self.send_input())
        
        tk.Button(self.input_frame, text="Send", command=self.send_input, bg="#4CAF50", fg="white").pack(side=tk.LEFT)
        
        # Quick test buttons
        button_frame = tk.Frame(root)
        button_frame.pack(pady=10)
        
        tk.Button(button_frame, text="Test: What is 2+2?", command=lambda: self.quick_send("What is 2+2?")).pack(side=tk.LEFT, padx=5)
        tk.Button(button_frame, text="Test: /write-direct", command=lambda: self.quick_send("/write-direct /tmp/test.txt Hello")).pack(side=tk.LEFT, padx=5)
        tk.Button(button_frame, text="Exit OCLI", command=lambda: self.quick_send("exit")).pack(side=tk.LEFT, padx=5)
        
        # Status
        self.status = tk.Label(root, text="Status: Not started", font=("Courier", 10), fg="red")
        self.status.pack(pady=5)
        
        # Start OCLI
        self.master_fd = None
        self.start_ocli()
        
    def start_ocli(self):
        self.master_fd, slave_fd = pty.openpty()
        self.proc = subprocess.Popen(
            ["/home/drew/projects/ollama-ocli/target/release/ocli"],
            stdin=slave_fd,
            stdout=slave_fd,
            stderr=slave_fd,
            close_fds=True
        )
        os.close(slave_fd)
        
        self.status.config(text="Status: OCLI Running", fg="green")
        
        # Read output thread
        threading.Thread(target=self.read_output, daemon=True).start()
        
    def read_output(self):
        while True:
            try:
                r, _, _ = select.select([self.master_fd], [], [], 0.1)
                if r:
                    data = os.read(self.master_fd, 1024).decode(utf-8, errors=ignore)
                    self.root.after(0, lambda d=data: self.append_output(d))
            except:
                break
                
    def append_output(self, text):
        self.output.insert(tk.END, text)
        self.output.see(tk.END)
        
    def send_input(self):
        text = self.input_box.get()
        if text:
            self.append_output(f"\n>>> {text}\n")
            os.write(self.master_fd, (text + "\n").encode())
            self.input_box.delete(0, tk.END)
            
    def quick_send(self, text):
        self.input_box.delete(0, tk.END)
        self.input_box.insert(0, text)
        self.send_input()

if __name__ == "__main__":
    root = tk.Tk()
    app = OCLITester(root)
    root.mainloop()
