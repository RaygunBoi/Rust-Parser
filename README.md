## Description
In Rust, this program takes another program written in DA, and outputs:
1. If the program has lexical or syntax errors, the error that was found. Use "hide the head in the sand, like an ostrich" version of error handling.
1. If the program is OK, depending on a command line flag the program will produce:
   1.	If the flag is `-s` the program will output a code in Scheme that is going to be called by a program in Scheme that will execute the operations specified in the program.
   1. If the flag is `-p` the program will output a series of queries based in the operations specified in the program.

The program should run like this:
```
prompt>cargo run input.da -s
; Processing Input File input.da
; Lexical and Syntax analysis passed
(define xvalues (read-csv "./file.csv" #f 0))
(define yvalues (read-csv "./file.csv" #f 1))
(define a (regressiona xvalues yvalues))
(define b (regressionb xvalues yvalues))
(define r (correlation xvalues yvalues))
(display "value of a = ")
(newline)
(display a)
(newline)
(display "value of b = ")
(newline)
(display b)
(newline)
(display "value of r = ")
(newline)
(display r)
(newline)
prompt>
```
Given the following program written in this language:
```
data:
   xvalues = vector,
   yvalues = vector,
   a = number,
   b = number,
   r = number
input:
   xvalues = read("file.csv", false, 0),
   yvalues = read("file.csv", false, 1)
process:
   a = regressiona(xvalues, yvalues),
   b = regressionb(xvalues, yvalues),
   r = correlation(xvalues, yvalues)
output:
   "value of a = ",
   a,
   "value of b = ",
   b,
   "value of r = ",
   r
end.
```
The tokens that it would generate are:
1. DATA
2. COLON
3. ID xvalues
4. ASSIGN
5. VECTOR 
6. COMMA
7. ID yvalues
8. ASSIGN
9. VECTOR
10. COMMA
11. ID a
12. ASSIGN
13. NUMBER
14. COMMA
15. ID b
16. ASSIGN
17. NUMBER
18. COMMA
19. ID r
20. ASSIGN
21. NUMBER
22. INPUT
23. COLON
24. ID xvalues
25. ASSIGN
26. READ
27. LPAREN
28. STRING "file.csv"
29. COMMA
30. FALSE
31. COMMA
32. NUM 0
33. RPAREN
34. COMMA 
35. ID yvalues
36. ASSIGN
37. READ
38. LPAREN
39. STRING "file.csv"
40. COMMA
41. FALSE
42. COMMA
43. NUM 1
44. RPAREN
45. PROCESS
46. COLON
47. ID a
48. ASSIGN
49. REGRESSIONA
50. LPAREN
51. ID xvalues
52. COMMA
53. ID yvalues
54. RPAREN
55. COMMA
56. ID b
57. ASSIGN
58. REGRESSIONB
59. LPAREN
60. ID xvalues
61. COMMA
62. ID yvalues
63. RPAREN
64. COMMA
65. ID r
66. ASSIGN
67. CORRELATION
68. LPAREN
69. ID xvalues
70. COMMA
71. ID yvalues
72. RPAREN
73. OUTPUT
74. COLON
75. STRING "value of a = "
76. COMMA
77. ID a
78. COMMA
79. STRING "value of b = "
80. COMMA
81. ID b
82. COMMA 
83. STRING "value of r = " 
84. COMMA
85. ID r
86. END
87. PERIOD

## How to run the program

### Scheme Output
To generate scheme output you will add the `-s` flag at the end of the command:
```
prompt> cargo run input.da -s
; processing input file input.da
; Lexical and Syntax analysis passed
(define xvalues (read-csv "./file.csv" #f 0))
(define yvalues (read-csv "./file.csv" #f 1))
(define a (regressiona xvalues yvalues))
(define b (regressionb xvalues yvalues))
(define r (correlation xvalues yvalues))
(display "value of a = ")
(newline)
(display a)
(newline)
(display "value of b = ")
(newline)
(display b)
(newline)
(display "value of r = ")
(newline)
(display r)
(newline)
```


### Prolog Output
To generate prolog output you will add the `-p` flag at the end of the command:
```
prompt> cargo run input.sc -p
/* processing input file input.sc
   Lexical and Syntax analysis passed */

main :-
   load_data_column('file.csv', false, 0, Data0),
   load_data_column('file.csv', false, 1, Data1),
   regressiona(Data0, Data1, A),
   regressionb(Data0, Data1, B),
   correlation(Data0, Data1, R),
   writeln("value of a = "),
   writeln(A),
   writeln("value of b = "),
   writeln(B),
   writeln("value of r = "),
   writeln(R).

```