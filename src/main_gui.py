import sys
import os
from PyQt6.QtCore import Qt

from PyQt6.QtWidgets import (
    QApplication,
    QComboBox,
    QFormLayout,
    QLineEdit,
    QStackedLayout,
    QVBoxLayout,
    QWidget,
    QTextEdit,
    QPushButton,
)

os.chdir('..')# для работы с SimpleDB2.exe

class Window(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("SimpleDB2")
        # Create a top-level layout
        layout = QVBoxLayout()
        self.setLayout(layout)
        
        # Create and connect the combo box to switch between pages
        self.pageCombo = QComboBox()
        self.pageCombo.addItems(["Информация", "Студент", "Посещаемость", "Направление", "Ведомость", "Тема занятия", "Предмет", "Факультет"])
        self.pageCombo.activated.connect(self.switchPage)
        
        # Create the stacked layout
        self.stackedLayout = QStackedLayout()
        
        
        # Create the page(информация)
        self.page1 = QWidget()
        self.page1Layout = QFormLayout()
        self.input = QTextEdit()#Сюда всё выводится
        self.page1Layout.addRow(self.input)
        self.button = QPushButton("Информация")#Кнопка для получения информации
        self.button.clicked.connect(self.Info)
        self.page1Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.page1.setLayout(self.page1Layout)
        self.stackedLayout.addWidget(self.page1)


        # Create the page(студент)
        self.page2 = QWidget()
        self.page2Layout = QFormLayout()
        self.input2 = QTextEdit()#Сюда всё выводится
        
        self.page2Layout.addRow(self.input2)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input2.clear)
        self.page2Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по студентам")#Кнопка для получения таблицы по студентам
        self.button.clicked.connect(self.StudOut)
        self.page2Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.StudIn)
        self.page2Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page2.setLayout(self.page2Layout)
        self.stackedLayout.addWidget(self.page2)
        
        
        # Create the page(посещаемость)
        self.page3 = QWidget()
        self.page3Layout = QFormLayout()
        self.input3 = QTextEdit()#Сюда всё выводится
        
        self.page3Layout.addRow(self.input3)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input3.clear)
        self.page3Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по посещаемости")#Кнопка для получения таблицы по посещаемости
        self.button.clicked.connect(self.PosOut)
        self.page3Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.PosIn)
        self.page3Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page3.setLayout(self.page3Layout)
        self.stackedLayout.addWidget(self.page3)
        
        # Create the page(направление)
        self.page4 = QWidget()
        self.page4Layout = QFormLayout()
        self.input4 = QTextEdit()#Сюда всё выводится
        
        self.page4Layout.addRow(self.input4)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input4.clear)
        self.page4Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по направленям")#Кнопка для получения таблицы по направленям
        self.button.clicked.connect(self.NapOut)
        self.page4Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.NapIn)
        self.page4Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page4.setLayout(self.page4Layout)
        self.stackedLayout.addWidget(self.page4)
        
        # Create the page(Ведомость)
        self.page5 = QWidget()
        self.page5Layout = QFormLayout()
        self.input5 = QTextEdit()#Сюда всё выводится
        
        self.page5Layout.addRow(self.input5)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input5.clear)
        self.page5Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по ведомости")#Кнопка для получения таблицы по ведомости
        self.button.clicked.connect(self.VedOut)
        self.page5Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.VedIn)
        self.page5Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page5.setLayout(self.page5Layout)
        self.stackedLayout.addWidget(self.page5)
        
        # Create the page(тема занятия)
        self.page6 = QWidget()
        self.page6Layout = QFormLayout()
        self.input6 = QTextEdit()#Сюда всё выводится
        
        self.page6Layout.addRow(self.input6)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input6.clear)
        self.page6Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по темам занатий")#Кнопка для получения таблицы по темам занятий
        self.button.clicked.connect(self.ZanOut)
        self.page6Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.ZanIn)
        self.page6Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page6.setLayout(self.page6Layout)
        self.stackedLayout.addWidget(self.page6)
        
        # Create the page(предмет)
        self.page7 = QWidget()
        self.page7Layout = QFormLayout()
        self.input7 = QTextEdit()#Сюда всё выводится
        
        self.page7Layout.addRow(self.input7)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input7.clear)
        self.page7Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по темам предметам")#Кнопка для получения таблицы по предметам
        self.button.clicked.connect(self.PredOut)
        self.page7Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.PredIn)
        self.page7Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page7.setLayout(self.page7Layout)
        self.stackedLayout.addWidget(self.page7)
        
        # Create the page(факультет)
        self.page8 = QWidget()
        self.page8Layout = QFormLayout()
        self.input8 = QTextEdit()#Сюда всё выводится
        
        self.page8Layout.addRow(self.input8)
        self.button = QPushButton("Очистить вывод")
        self.button.clicked.connect(self.input8.clear)
        self.page8Layout.addRow(self.button)#добавление кнопки с очисткой вывода
        
        self.button = QPushButton("Получить таблицу по темам факультетам")#Кнопка для получения таблицы факультетам
        self.button.clicked.connect(self.FacOut)
        self.page8Layout.addRow(self.button)#добавление кнопки с выводом
        
        self.button = QPushButton("Ввод новых данных(Внимание! Ввод берется из поля с текстом)")#Ввод
        self.button.clicked.connect(self.FacIn)
        self.page8Layout.addRow(self.button)#добавление кнопки с вводом
        
        self.page8.setLayout(self.page8Layout)
        self.stackedLayout.addWidget(self.page8)

        # Add the combo box and the stacked layout to the top-level layout
        layout.addWidget(self.pageCombo)
        layout.addLayout(self.stackedLayout)
        
        #Функции
    def switchPage(self):
        self.stackedLayout.setCurrentIndex(self.pageCombo.currentIndex())
        
    def Info(self):
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('Help.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input.setText(text)
        
    def StudOut(self):
        os.system(".\SimpleDB2.exe -S -t Студент > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input2.setText(text)
        
    def StudIn(self):
        change=".\SimpleDB2.exe -I -t Студент -v "
        change+=(self.input2.toPlainText())
        os.system(change)      

    def PosOut(self):
        os.system(".\SimpleDB2.exe -S -t Посещаемость > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input3.setText(text)
        
    def PosIn(self):
        change=".\SimpleDB2.exe -I -t Посещаемость -v "
        change+=(self.input3.toPlainText())
        os.system(change)
        
    def NapOut(self):
        os.system(".\SimpleDB2.exe -S -t Направление > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input4.setText(text)
        
    def NapIn(self):
        change=".\SimpleDB2.exe -I -t Направление -v "
        change+=(self.input4.toPlainText())
        os.system(change) 
        
    def VedOut(self):
        os.system(".\SimpleDB2.exe -S -t Ведомость > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input5.setText(text)
        
    def VedIn(self):
        change=".\SimpleDB2.exe -I -t Ведомость -v "
        change+=(self.input5.toPlainText())
        os.system(change)
        
    def ZanOut(self):
        os.system('.\SimpleDB2.exe -S -t "Тема занятия" > stud.txt')       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input6.setText(text)
        
    def ZanIn(self):
        change='.\SimpleDB2.exe -I -t "Тема занятия" -v '
        change+=(self.input6.toPlainText())
        os.system(change)
        
    def PredOut(self):
        os.system(".\SimpleDB2.exe -S -t Предмет > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input7.setText(text)
        
    def PredIn(self):
        change=".\SimpleDB2.exe -I -t Предмет -v "
        change+=(self.input7.toPlainText())
        os.system(change)
        
    def FacOut(self):
        os.system(".\SimpleDB2.exe -S -t Факультет > stud.txt")       #выполнение команды в БД
        text_edit = QTextEdit()                                                        #Копирование текста и кодировка
        text=open('stud.txt',encoding='utf-8').read()
        text_edit.setPlainText(text)
        self.input8.setText(text)
        
    def FacIn(self):
        change=".\SimpleDB2.exe -I -t Факультет -v "
        change+=(self.input8.toPlainText())
        os.system(change) 

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = Window()
    window.show()
    sys.exit(app.exec())