from PyQt6.QtWidgets import QApplication, QWidget, QTextEdit, QVBoxLayout, QPushButton,QStackedLayout
from PyQt6.QtCore import Qt
import sys
import os

os.chdir('..')

class FormA(QWidget):
    def __init__(self, parent=None):
        super().__init__()
        layout = QVBoxLayout()
        self.setLayout(layout)

        self.input = QTextEdit()#Сюда всё выводится
        #self.input.setFixedWidth(500)
        layout.addWidget(self.input)

        self.button = QPushButton("Очистить вывод")#Всё и так понятно
        self.button.clicked.connect(self.input.clear)
        layout.addWidget(self.button)

        self.button = QPushButton("Получить таблицу по студентам")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.wtf)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)
        
        self.btnPress = QPushButton("Страница1")
        layout = QVBoxLayout()

        layout.addWidget(self.btnPress)

    def wtf(self):
        os.system("cd ..");os.system(".\SimpleDB2.exe -S -t Студент > stud.txt")       #Переход в директиву выше и выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t Студент -v "
        change+=(self.input.toPlainText())
        
        os.system("cd..");os.system(change)             #".\SimpleDB2.exe -I -t Студент -v "

class FormB(QWidget):
    def __init__(self, parent=None):
        super().__init__()
        layout = QVBoxLayout()
        self.setLayout(layout)
        
        self.input = QTextEdit()#Сюда всё выводится
        #self.input.setFixedWidth(500)
        layout.addWidget(self.input)
        
        self.button = QPushButton("Очистить вывод")#Всё и так понятно
        self.button.clicked.connect(self.input.clear)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Получить таблицу по факультетам")#Кнопка для получения таблицы 
        self.button.clicked.connect(self.tab)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)

    def tab(self):
        os.system("cd ..");os.system(".\SimpleDB2.exe -S -t Факультет > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t Факультет -v "
        change+=(self.input.toPlainText())
        
        os.system("cd..");os.system(change) 

class Window(QWidget):
    def __init__(self, parent=None):
        super(Window, self).__init__(parent)
        self.setWindowTitle("Основное окно для БД")
        self.resize(500, 500)
        layout = QVBoxLayout()
        
        self.setLayout(layout)
        
        widget = QWidget()
        self.stacked_layout = QStackedLayout()
        widget.setLayout(self.stacked_layout)
        layout.addWidget(widget)
#########первая страница - по студентам
        self.btnPress1 = QPushButton("Студенты")
        self.form1 = FormA()
        layout.addWidget(self.btnPress1)
        self.btnPress1.clicked.connect(self.btnPress1_Clicked)
#########вторая страница - по чтотото
        self.btnPress2 = QPushButton("Факультеты")
        self.form2 = FormB()
        layout.addWidget(self.btnPress2)
        self.btnPress2.clicked.connect(self.btnPress2_Clicked)

        self.stacked_layout.addWidget(self.form1)
        self.stacked_layout.addWidget(self.form2)

    def btnPress1_Clicked(self):
        self.stacked_layout.setCurrentIndex(0)

    def btnPress2_Clicked(self):
        self.stacked_layout.setCurrentIndex(1)

    

if __name__ == "__main__":
    app = QApplication(sys.argv)
    win = Window()
    win.show()
    sys.exit(app.exec())
