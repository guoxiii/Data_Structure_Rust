/*
    中序轉後序，可依下列三步驟進行
    1、將運算式中的運算單元加上括號，需運算優先順序
    2、將所有的運算子移動到其對應括號的右邊
    3、將所有的括號去除

    運算子的優先順序(由高到低)
        +(正)、-(負)
        *、/、%
        +(加)、-(減)
        <、<=、>、>=
        &&
        ||

    範例一
        中序：A*B+C
        後序：
            -> ((A*B)+C)
            -> ((AB)*C)+
            -> AB*C+

    範例二
        中序：A-B/C+D*E-F%G
        後序：
            -> (((A-(B/C))+(D*E))-(F%G))
            -> (((A(BC)/)-(DE)*)+(FG)%)-
            -> ABC/-DE*+FG%-

    範例三
        中序：A+B*C
        後序：
            -> (A+(B*C))
            -> (A(BC)*)+
            -> ABC*+

    範例四
        中序：A*(B+C)*D
        後序：
            -> ((A*(B+C))*D)
            -> ((A(BC)+)*D)*
            -> ABC+*D*

    範例五
        中序：-A*(B+C)*-D
        後序：
            -> (-A)*(B+C)*(-D)
            -> (((-A)*(B+C))*(-D))
            -> (((A)-(BC)+)*(D)-)*
            -> A-BC+*D-*
*/

use code::Priority;

fn main() {
    println!("中序\t\t\t後序");
    println!("----------------------------------------------------");

    print!("A*B+C\t\t\t");
    Priority::new("A*B+C").show();          // AB*C+

    print!("A-B/C+D*E-F%G\t\t");
    Priority::new("A-B/C+D*E-F%G").show();  // ABC/-DE*+FG%-

    print!("A+B*C\t\t\t");
    Priority::new("A+B*C").show();          // ABC*+

    print!("A*(B+C)*D\t\t");
    Priority::new("A*(B+C)*D").show();      // ABC+*D*

    print!("-A*(B+C)*-D\t\t");
    Priority::new("-A*(B+C)*-D").show();    // A-BC+*D-*
}