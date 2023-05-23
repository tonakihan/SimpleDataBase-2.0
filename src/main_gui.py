from PyQt6.QtWidgets import (QApplication, QWidget, QLineEdit, QPushButton, QVBoxLayout)
from PyQt6.QtCore import Qt
import sys

class Window(QWidget):
    def __init__(self):
        super().__init__()
        self.resize(300, 250)
        self.setWindowTitle("I like cats")
 
        layout = QVBoxLayout()
        self.setLayout(layout)
 
        self.input = QLineEdit()
        self.input.setFixedWidth(150)
        layout.addWidget(self.input, alignment= Qt.AlignmentFlag.AlignCenter)
 
        button = QPushButton("Get Text")
        button.clicked.connect(self.get)
        layout.addWidget(button)
 
        button = QPushButton("Clear Text")
        button.clicked.connect(self.input.clear)
        layout.addWidget(button)
 
    def get(self):
        text = self.input.text()
        print(text)
 
app = QApplication(sys.argv)
window = Window()
window.show()
sys.exit(app.exec())
