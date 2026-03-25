#include <iostream>
#include <cstring>
using namespace std;

struct book;
struct s_node;
struct q_node;



class book
{
  public:
    int copy_entry(const book & copy_from); 
    int display();
    
  public:
    char * author; 
    char * title; 
    char * publisher; 
    int pageleftoff; 
};

struct s_node 
{
  book * object;
  s_node * next;
  int top_index;

};

int book::copy_entry(const book & copy_from) 
{

  if (author)
   delete [] author; 
  if (title) 
   delete [] title;
  if (publisher) 
   delete [] publisher; 
  if (pageleftoff) 
   pageleftoff = 0; 
     
  title = author = publisher = NULL; 
  pageleftoff = 0;

  if (copy_from.author)
  { 
     author = new char[strlen(copy_from.author) +1]; 
     strcpy(author, copy_from.author); 
  } 
  if (copy_from.title) 
  { 
     title = new char[strlen(copy_from.title)+1]; 
     strcpy(title, copy_from.title); 
  } 
  
  if (copy_from.publisher)
  { 
     publisher = new char[strlen(copy_from.publisher) +1]; 
     strcpy(publisher, copy_from.publisher); 
  } 
  if (copy_from.pageleftoff) 
  { 
     pageleftoff = copy_from.pageleftoff;
  } 

  return 1;
}

int book::display() 
{
  cout << title << endl;
  cout << author << endl;
  cout << publisher << endl;
  cout << pageleftoff << endl;
  
  return 1;

}

class stack
{
  public:
    stack(); 
    ~stack(); 
    int push(book & data); 
    int pop(); 
    int display(); 
    int peek(); 
    int copy_stack(const stack & data); 
  private:
    s_node * head; 
    int display(s_node * & head);
  

};
stack::stack()
{
  head = NULL; 
}

stack::~stack()
{

  s_node * temp = NULL;
  if(head)
  {
    while(head->next != NULL)
    {
      temp = head->next; 
      delete head;
      head = temp; 
    }
  }
  

}

int stack::copy_stack(const stack & data) 
{
  s_node * dcurrent = data.head;
  s_node * current = head;
  while(dcurrent)
  {
    current = new s_node;
    current->next = NULL;
    for(int i = 0; i <= dcurrent->top_index-1; ++i){
      int show = push(dcurrent->object[i]);} 
    dcurrent = dcurrent->next;
    current = current->next;
    
  }
  
  return 1;
}


int stack::push(book & data)
{
  s_node * temp;
  int MAX = 2;
  if(!head) 
  {
    head = new s_node;  
    head->object = new book[MAX]; 
    head->next = NULL; 
    head->top_index = 0; 
  }
  
  if(head->top_index == MAX) 
  {
    temp = head;
    head = new s_node;
    head->object = new book[MAX]; 
    head->next = temp; 
    head->top_index = 0; 
  }
  
  if (head->object[head->top_index].copy_entry(data)) 
     ++head->top_index; 
  return 1;
}

int stack::pop()
{
  s_node * hold = head->next; 
  if (!head->top_index) 
  { 
    if (head->object) 
      delete [] head->object; 
    delete head; 
    head = hold; 
    head->top_index = 2; 
  }
  --head->top_index;
  return 1;
}


int stack::peek() 
{
  if(!head)
    return 0;
  else{
    for(int i = head->top_index-1; i > head->top_index-2; --i){
      int show = head->object[i].display();}

  return 1;}
}

int stack::display()
{

  return display(head); 
}
int stack::display(s_node * & head)
{
  if(!head)
    return 0;
  else
  {
    for(int i = head->top_index-1; i > -1; --i){
       int show = head->object[i].display();} 
  }
  return 1 + display(head->next); 
  
}

book MakeABook()
{
  book bookToAdd;
  
  char bookName[50];
  char authorName[50];
  char publisherName[50];
  int whereLeftOff{0};
  
  cout << "Enter the name of a book: ";
  cin.get(bookName, 100, '\n');
  cin.ignore(100, '\n');
  cout << endl;
  
  cout << "Enter the author of the book: ";
  cin.get(authorName, 100, '\n');
  cin.ignore(100, '\n');
  cout << endl;
  
  cout << "Enter the publisher of the book: ";
  cin.get(publisherName, 100, '\n');
  cin.ignore(100, '\n');
  cout << endl;
  
  cout << "Enter the last page you left off on: ";
  cin >> whereLeftOff;
  cin.ignore(100, '\n');
  cout << endl;
  
  char * dynamicBookName = new char[strlen(bookName) + 1]; 
  strcpy(dynamicBookName, bookName); 
  
  char * dynamicAuthorName = new char[strlen(authorName) + 1]; 
  strcpy(dynamicAuthorName, authorName); 
  
  char * dynamicPublisherName = new char[strlen(publisherName) + 1]; 
  strcpy(dynamicPublisherName, publisherName); 
  
  bookToAdd.title = dynamicBookName;
  bookToAdd.author = dynamicAuthorName;
  bookToAdd.publisher = dynamicPublisherName;
  bookToAdd.pageleftoff = whereLeftOff;
  
  
  return bookToAdd;
}


int main()
{
  book elements[2];
  stack books;
  int decider{0};
  cout << "Welcome, please select what you would like to accomplish today:\nCreate a stack of books(1)" << endl
  << "Terminating the program(0): ";
  cin >> decider;
  cin.ignore(100, '\n');
  cout << endl;
  
  
  while(decider != 0) 
  {
    if(decider == 1) 
    {
      int add{0};
      book addition = MakeABook();
      add = books.push(addition); 
      
    }
    if(decider == 2) 
    {
      int show = books.display();
    }
    
    if(decider == 3) 
    {
      int show = books.pop();
    }
    
    
    if(decider == 4)
    {
      int show = books.peek(); 
    }
    
    
    cout << "Select your next action: \nAdding another book to a stack of books(1)\nDisplaying all books in a stack(2)\nRemoving a book from the top of the stack(3)\nPeek the book at the top of a book stack(4)" 
    << endl << "\nTerminating the program(0)"; 
    cin >> decider;
    cin.ignore(100, '\n'); 
  }
  
  cout << "Thank you for using the program, good day!" << endl << endl << endl; 


  return 0;
}