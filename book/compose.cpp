/*
    石の置き方
    黒から見たfinal game result\n
    を繰り返して出力
*/
#include <bits/stdc++.h>
using namespace std;

int print_uint128(__uint128_t n){ //1文字
      if (n == 0)  return printf("0\n");

      char str[40] = {0}; // log10(1 << 128) + '\0'
      char *s = str + sizeof(str) - 1; // start at the end
      while (n != 0) {
        if (s == str) return -1; // never happens

        *--s = "0123456789"[n % 10]; // save last digit
        n /= 10;                     // drop it
      }
      return printf("%s", s);
    }

int main() {
    ifstream ifs("xxx.gam");

    string str, result_in_black_view, end10;
    //int skip = 0;
    __uint128_t  val = 0;

    __uint128_t  now_val = 0;
    __uint128_t  grouping = 0;
    int flag = 0;
    int sum = 0;
    int count = 1;
    while (ifs >> str >> result_in_black_view >> end10) {
        //skip++;
        //if (skip%2) continue;
        int cnt = 0;
        int len = 0;
        val = 0;
        for (int i = 0; i < str.size(); i++) {
            if (str[i] != '-' && str[i] != '+' && str[i] != ':') {
                cnt++;
                if (cnt == 1) {
                    val = (val<<3) + (str[i]-'a');
                } else {
                    val = (val<<3) + (str[i]-'1');
                    len++;
                }
            } else {
                cnt = 0;
            }
            if (len >= 21) {
                now_val = val;
                if (now_val != grouping) {
                    if (flag==1){
                        print_uint128(grouping);
                        cout << " " << (float)sum/(float)count << endl;
                    }
                    flag = 1;
                    grouping = now_val;
                    count = 1;
                    sum = std::stoi(result_in_black_view);
                }else{
                    count ++;
                    sum += std::stoi(result_in_black_view);
                }

                break;
            };
        }



    }
}
