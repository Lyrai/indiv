# Приведение лямбда выражений к нормальной форме
## Синтаксис
Алфавит: {a-zA-Z, ^, ., (, )}
Имена переменных состоят из одного символа a-zA-Z
Корректным выражением является любой корректный лямбда терм. Пробелы игнорируются.
Не распознает выражения вида ```^xy.(...)```, каждая лямбда абстракция пишется отдельно: ```^x.^y.(...)```.
Порядок операций такой же как в лямбда исчислении, приоритет распознается автоматически и изменяется скобками.
#### Примеры:
```
^x.x
(^x.xx)(^x.xx)
(^x.^y.yx)uv
(^x.^y.yx)(uv)
```
## Возможности
- Построение вывода нормальной формы по шагам со всеми заменами
- Обнаружение циклов

## Использование
Собрать из исходников либо скачать из папки build исполняемый файл и запустить. Откроется командная строка в которую можно вводить примеры для приведения к нормальной форме. Когда пример введен, нажать Enter, на экране появится результат построения и приглашение ввести новый пример. Для выхода ввести quit.
