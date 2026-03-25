#include <iostream>

using namespace std;

struct node 
{
    int data;
    node * next;

};

class LLL
{
    public:
        int insert(int data);
        int display();
        int reverseList();
    private:
        node * head;
        int insert(int data, node *& current);
        int display(node *& current);
        int reverseList(node *& current);

};

int LLL::insert(int data)
{
    if(!head) {
        head = new node;
        head->data = data;
        head->next = nullptr;
        return 1;
    }
    return insert(data, head->next);

}

int LLL::insert(int data, node *& current)
{
    if(!current) {
        current = new node;
        current->data = data;
        current->next = nullptr;
        return 1;
    }
    return insert(data, current->next);

}

int LLL::display()
{
    if(!head) return 0;

    return display(head);

}
int LLL::display(node *& current)
{
    if(!current) return 0;

    cout << current->data << endl;
    return display(current->next);


}

int LLL::reverseList()
{
    if(!head) return 0;
    node * current = head;
    return reverseList(current);
}

int LLL::reverseList(node *& current)
{
    node *prev = nullptr;
    node *next = nullptr;

    while(current != nullptr)
    {
        next = current->next;
        current->next = prev;
        prev = current;
        current = next;

    }
    head = prev;
    return 1;

}

int main() 
{

    LLL mylist;
    
    mylist.insert(1);
    mylist.insert(2);
    mylist.insert(3);
    mylist.insert(4);
    
    mylist.display();
    mylist.reverseList();
    mylist.display();
    

    return 0;
}

