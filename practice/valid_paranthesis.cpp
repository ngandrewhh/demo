class Solution {
public:
    bool isValid(string s) {
        
		stack<char> stk;
        for (auto c: s) {
            switch (c){
                case '(':
                case '{':
                case '[':
                    cout << "pushed left" << endl;
                    stk.push(c);
                    break;
					
                case ')':
                case '}':
                case ']':
                    if (stk.size() > 0) 
					{
                        cout << "found right, pop left" << endl;
                        char x = stk.top();
                        stk.pop();
                        if ((c == ')') and (x != '(')) return false;
                        if ((c == ']') and (x != '[')) return false;
                        if ((c == '}') and (x != '{')) return false;
					}
					else 
					{
                       cout << "cannot find left" << endl;
                       return false;
					}
                break;
            }
        }
		
        if (stk.size() != 0) return false;
        return true;
    }
};
