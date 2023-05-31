from PyQt6.QtWidgets import (QApplication, QWidget, QLineEdit, QPushButton, QVBoxLayout, QTextEdit)
from PyQt6.QtCore import Qt
import sys
import os

class Window(QWidget):
    def __init__(self):
        super().__init__()
        self.resize(500, 500)
        self.setWindowTitle("Why are we still here? Just to suffer?")
 
        layout = QVBoxLayout()
        self.setLayout(layout)
 
        self.output = QTextEdit()
        self.output.setFixedWidth(500)
        layout.addWidget(self.output, alignment= Qt.AlignmentFlag.AlignCenter)
 
        self.input = QLineEdit()
        self.input.setFixedWidth(200)
        layout.addWidget(self.input, alignment= Qt.AlignmentFlag.AlignCenter)
 
        button = QPushButton("Get Text")
        button.clicked.connect(self.get)
        layout.addWidget(button)
 
        button = QPushButton("Clear Text")
        button.clicked.connect(self.input.clear)
        layout.addWidget(button)

        button = QPushButton("Студент")
        button.clicked.connect(self.wtf)
        layout.addWidget(button)

        button = QPushButton("Посещаемость")
        button.clicked.connect(self.get)
        layout.addWidget(button)

        button = QPushButton("Направление")
        button.clicked.connect(self.get)
        layout.addWidget(button)

        button = QPushButton("Ведомость")
        button.clicked.connect(self.get)
        layout.addWidget(button)

        button = QPushButton("Тема занятия")
        button.clicked.connect(self.get)
        layout.addWidget(button)

        button = QPushButton("Предмет")
        button.clicked.connect(self.get)
        layout.addWidget(button)

        button = QPushButton("Факультет")
        button.clicked.connect(self.get)
        layout.addWidget(button)
 
    def get(self):
        text = self.input.text()
        print(text)

    def wtf(self):
        os.system("C:/Users/shevc/SimpleDataBase-2.0/SimpleDB2.exe -S -t Студент > stud.txt")
        text_edit = QTextEdit()
        text=open('stud.txt').read()
        text_edit.setPlainText(text)
 
def main():
    app = QApplication(sys.argv)
    window = Window()
    window.show()
    sys.exit(app.exec())

if __name__=="__main__":
    main()