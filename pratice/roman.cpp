class Solution {
public:
    
    // parsing right to left
  
    int romanToInt(string s) {
      
        int sum = 0;
        string prev = "I";
        for (int i = s.size() - 1; i >= 0; i--) {
          
    // reading cppref using fill (6)	string (size_t n, char c);
          
        string curr( 1, s[i] );
          
            if (roman[prev] > roman[curr] ) {
                sum -= roman[curr];
            } else {
                sum += roman[curr];
            }
            prev = curr;
        }
        return sum;
    }
    
    // STL Hash Map
    map<string, int> roman = {{"I", 1}, {"V", 5}, {"X", 10}, {"L", 50}, {"C", 100}, {"D", 500}, {"M", 1000}};
};
