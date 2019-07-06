/*
    +d3-c3は
    +3 +2 -2 -2 などとして出力したいが、
    0 0のときが区別できないので、全てに+1をしておく

    60手よりも前に終わった場合は10で穴埋め
*/
#include <bits/stdc++.h>
using namespace std;



int main() {
    ifstream ifs("../data/xxx.gam");
    ofstream ofs("../train/pre_train_data.csv");

    string str, result_in_black_view, end10;
    int flag=0;// 黒の手なら0
    while (ifs >> str >> result_in_black_view >> end10) {
        int cnt = 0;
        int val = 0;
        int count60 = 0;
        for (int i = 0; i < str.size(); i++) {
            if (str[i] != '-' && str[i] != '+' && str[i] != ':') {
                cnt++;
                if (cnt == 1) {
                    val = (str[i]-'a')+1;
                    if(flag==0){
                        ofs << val << " ";
                    }else{
                        ofs << -val << " ";
                    }
                } else {
                    val = (str[i]-'1')+1;
                    count60++;
                    if(flag==0){
                        ofs << val << " ";
                    }else{
                        ofs << -val << " ";
                    }
                }
            } else if(str[i] == '+'){
                flag=0;
                cnt = 0;
            } else if(str[i] == '-'){
                flag=1;
                cnt = 0;
            }
        }

        for(; count60<60;count60++){
            ofs << 10 << " " << 10 << " ";
        }

        ofs << result_in_black_view << endl;

    }
}
