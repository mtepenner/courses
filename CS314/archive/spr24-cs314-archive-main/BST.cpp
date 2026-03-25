//Matthew Penner

#include <iostream>
#include <cstring>

using namespace std;
struct node
{
  node * left;
  node * right;
  int data;

};

class BST
{
  public:
    BST();
    ~BST();
    int insert(int toInsert);
    int removeAll();
    int displayAll(); //Displays in level order
  private:
    int insert(node *& current, int toInsert);
    int displayAll(node *& current);
    int removeAll(node * & current);
 
    
    node * root;
};



BST::BST()
{
  root = new node;
  root->left = NULL;
  root->right = NULL;
  root = NULL;
}
BST::~BST() 
{
   removeAll(root); 
}

int BST::displayAll() 
{
  return displayAll(root); 
}
int BST::displayAll(node * & root) 
{
  if(!root) 
    return 0;
  else
  {
    displayAll(root->left);
    cout << root->data << endl;
    displayAll(root->right); 
    
    
  }
  return 1;
}

int BST::insert(int toInsert) 
{
  return insert(root, toInsert); 

}
int BST::insert(node *& current, int toInsert) 
{
  int add{0};
  if(!current) 
  {
    current = new node;
    current->data = toInsert;
    current->left = current->right = NULL;
    return 1;
  }
  if(current->data < toInsert) 
    return insert(current->right, toInsert);
  if(current->data > toInsert)
    return insert(current->left, toInsert);
  else
  {
    if(current->data < toInsert)
      return insert(current->right, toInsert);
    else
      return insert(current->left, toInsert);
  }
  current = new node;
  current->data = toInsert;
  
  return 1;
}


int BST::removeAll() 
{
    return removeAll(root);
}
int BST::removeAll(node * & current) 
{
  if (!current) return 0;
  int getout = removeAll(current->left)+removeAll(current->right); 
  delete current;
  
  return 1;
}  






int main() 
{
  
  BST binaryTable;
  
  binaryTable.insert(1);
  binaryTable.insert(2);
  binaryTable.insert(3);
  binaryTable.insert(4);
  binaryTable.insert(5);
  binaryTable.insert(6);
  binaryTable.insert(7);
  binaryTable.insert(8);
  binaryTable.insert(9);
  binaryTable.insert(10);
  binaryTable.insert(11);
  binaryTable.insert(12);
  binaryTable.insert(13);
  binaryTable.insert(14);
  
  
  
  binaryTable.displayAll();
  

  
  return 0;
}

























