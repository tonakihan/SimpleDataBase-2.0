from PyQt6.QtWidgets import QApplication, QWidget, QTextEdit, QVBoxLayout, QPushButton,QStackedLayout
from PyQt6.QtCore import Qt
import sys
import os

os.chdir('..')# для работы с SimpleDB2.exe

#Студенты
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки

        self.button = QPushButton("Получить таблицу по студентам")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.wtf)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)

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

#Факультеты
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки
        
        
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

#Направления
class FormC(QWidget):
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки
        
        self.button = QPushButton("Получить таблицу по направлениям")#Кнопка для получения таблицы 
        self.button.clicked.connect(self.tab)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)
        
    def tab(self):
        os.system("cd ..");os.system(".\SimpleDB2.exe -S -t Направление > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t Направление -v "
        change+=(self.input.toPlainText())
        
        os.system("cd..");os.system(change) 

#Ведомость      
class FormD(QWidget):
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки
        
        self.button = QPushButton("Получить таблицу по ведомости")#Кнопка для получения таблицы 
        self.button.clicked.connect(self.tab)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)
        
    def tab(self):
        os.system("cd ..");os.system(".\SimpleDB2.exe -S -t Ведомость > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t Направление -v "
        change+=(self.input.toPlainText())
        
        os.system("cd..");os.system(change)

#Темы занятий
class FormE(QWidget):
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки
        
        self.button = QPushButton("Получить таблицу по темам занятий")#Кнопка для получения таблицы 
        self.button.clicked.connect(self.tab)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)
        
    def tab(self):
        os.system("cd ..");os.system('.\SimpleDB2.exe -S -t "Тема занятия" > stud.txt')       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t 'Тема занятия' -v "
        change+=(self.input.toPlainText())
        
        os.system("cd..");os.system(change)             #".\SimpleDB2.exe -I -t Студент -v "

#Посещаемость
class FormF(QWidget):
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки
        
        self.button = QPushButton("Получить таблицу по посещаемости")#Кнопка для получения таблицы 
        self.button.clicked.connect(self.tab)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)
        
    def tab(self):
        os.system("cd ..");os.system(".\SimpleDB2.exe -S -t Посещаемость > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t Посещаемость -v "
        change+=(self.input.toPlainText())
        
        os.system("cd..");os.system(change)
        
#Предметы
class FormG(QWidget):
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
        self.button.setStyleSheet('QPushButton {background-color: #A3C1DA}')#цвет кнопки
        
        self.button = QPushButton("Получить таблицу по предметам")#Кнопка для получения таблицы 
        self.button.clicked.connect(self.tab)
        layout.addWidget(self.button)
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#
        self.button.clicked.connect(self.ins)
        layout.addWidget(self.button)
        
    def tab(self):
        os.system("cd ..");os.system(".\SimpleDB2.exe -S -t Предмет > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def ins(self):
        change=".\SimpleDB2.exe -I -t Предмет -v "
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
        self.btnPress1 = QPushButton("Студент")
        self.form1 = FormA()
        layout.addWidget(self.btnPress1)
        self.btnPress1.clicked.connect(self.btnPress1_Clicked)
#########вторая страница - по факультетам
        self.btnPress2 = QPushButton("Факультет")
        self.form2 = FormB()
        layout.addWidget(self.btnPress2)
        self.btnPress2.clicked.connect(self.btnPress2_Clicked)
#########третья страница - по направлениям 
        self.btnPress3 = QPushButton("Направление")
        self.form3 = FormC()
        layout.addWidget(self.btnPress3)
        self.btnPress3.clicked.connect(self.btnPress3_Clicked)
#########четвертая страница - по ведомости
        self.btnPress4 = QPushButton("Ведомость")
        self.form4 = FormD()
        layout.addWidget(self.btnPress4)
        self.btnPress4.clicked.connect(self.btnPress4_Clicked)
#########пятая страница - по темам занятий
        self.btnPress5 = QPushButton("Тема занятия")
        self.form5 = FormE()
        layout.addWidget(self.btnPress5)
        self.btnPress5.clicked.connect(self.btnPress5_Clicked)
#########шестая страница - по посещаемости
        self.btnPress6 = QPushButton("Посещаемость")
        self.form6 = FormF()
        layout.addWidget(self.btnPress6)
        self.btnPress6.clicked.connect(self.btnPress6_Clicked)
#########седьмая страница - по предметам 
        self.btnPress7 = QPushButton("Предмет")
        self.form7 = FormG()
        layout.addWidget(self.btnPress7)
        self.btnPress7.clicked.connect(self.btnPress7_Clicked)

#########Расположение кнопок
        self.stacked_layout.addWidget(self.form1)
        self.stacked_layout.addWidget(self.form2)
        self.stacked_layout.addWidget(self.form3)
        self.stacked_layout.addWidget(self.form4)
        self.stacked_layout.addWidget(self.form5)
        self.stacked_layout.addWidget(self.form6)
        self.stacked_layout.addWidget(self.form7)
        
#########индексы страниц (порядок)
    def btnPress1_Clicked(self):
        self.stacked_layout.setCurrentIndex(0)

    def btnPress2_Clicked(self):
        self.stacked_layout.setCurrentIndex(1)

    def btnPress3_Clicked(self):
        self.stacked_layout.setCurrentIndex(2)

    def btnPress4_Clicked(self):
        self.stacked_layout.setCurrentIndex(3)
        
    def btnPress5_Clicked(self):
        self.stacked_layout.setCurrentIndex(4)
        
    def btnPress6_Clicked(self):
        self.stacked_layout.setCurrentIndex(5)
        
    def btnPress7_Clicked(self):
        self.stacked_layout.setCurrentIndex(6)

    
#########"начало и конец"
if __name__ == "__main__":
    app = QApplication(sys.argv)
    win = Window()
    win.show()
    sys.exit(app.exec())