#include <iostream>

using namespace std;

struct node 
{
    int data;
    node * next;

};

class ARR
{
    public:
        int insert(int data);
        int display();
        int removeall();
        int reverseList();
    private:
        node * head;
        int insert(int data, node *& current);
        int display(node *& current);
        int removeall(node *& current);
        int reverseList(node *& current);
};

int ARR::insert(int data)
{
    if(!head) {
        head = new node;
        head->data = data;
        head->next = nullptr;
        return 1;
    }
    return insert(data, head->next);

}
int ARR::removeall()
{
    if(!head) return 0;
    return removeall(head);
}

int ARR::removeall(node *& current)
{
    if(!current) return 0;
    node * temp = current->next;
    delete current;
    current = temp;
    return removeall(current->next);

}


int ARR::reverseList()
{
    if(!head) return 0;
    node * current = head;
    return reverseList(current);
}

int ARR::reverseList(node *& current)
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


int ARR::insert(int data, node *& current)
{
    if(!current) {
        current = new node;
        current->data = data;
        current->next = nullptr;
        return 1;
    }
    return insert(data, current->next);

}

int ARR::display()
{
    if(!head) return 0;

    return display(head);

}
int ARR::display(node *& current)
{
    if(!current) return 0;

    cout << current->data << endl;
    return display(current->next);


}
int main() 
{

    ARR mylist[25];
    
    mylist[0].insert(1);
    mylist[0].insert(2);
    mylist[0].insert(3);
    mylist[0].insert(4);
    mylist[0].insert(5);
    
    mylist[0].display();
    cout << endl;

    mylist[0].reverseList();
    mylist[0].display();

    return 0;
}

