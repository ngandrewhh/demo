/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        
        bool b1 = (l1 != nullptr), b2 = (l2 != nullptr);
        ListNode *ptr1 = l1, *ptr2 = l2, *head = nullptr, *ptrh = nullptr;
        
        if      (!b1 && !b2) {return nullptr;}
        else if ( b1 && !b2) {return l1;}
        else if (!b1 &&  b2) {return l2;}
        else if ( b1 &&  b2) {head = new ListNode(); ptrh = head;}
    
        // Construct New List
        while (b1 || b2) 
        {
            
            if (b1 && !b2) 
            {
                ptrh->val = ptr1->val;
                ptr1 = ptr1->next;
            }
            
            else if (!b1 && b2) 
            {
                ptrh->val = ptr2->val;
                ptr2 = ptr2->next;
            }
            
            else  
            {
                if (ptr1->val < ptr2->val)
                {
                    cout << ptr1->val << "<" << ptr2->val << " true" << endl;
                    ptrh->val = ptr1->val;
                    ptr1 = ptr1->next;
                } 
                else
                {
                    cout << ptr1->val << "<" << ptr2->val << " false" << endl;
                    ptrh->val = ptr2->val;
                    ptr2 = ptr2->next;
                }
            }
            
            b1 = (ptr1 != nullptr), b2 = (ptr2 != nullptr);
            if (b1 || b2) {
                ptrh->next = new ListNode();
                ptrh = ptrh->next;
            }
        }
        return head;
    }
    
    void printll(ListNode* l) {
        cout << "[";
        while (l) {
            cout << l->val;
            l = l->next;
            if (l != nullptr) cout << ",";
        }
        cout << "]" << endl;
    }
};
