//
// Created by Dekai WU and Serkan Kumyol on 20190307.
//
#include "assignment.hpp"

using namespace std;

// TODO: put your student ID in this variable
const char* STUDENT_ID = "20354380";

// TODO: define your global variables and helper functions here if you need
#include <utility>
#include <iostream>
#include <typeinfo>
#include <queue>
#include <cmath>
#include <algorithm>
int MAX_ANAGRAM = 5;
int NPREFIX = 1;
int IS_LOCAL = false;
language_model* plm = nullptr;

// vector<double> uni(26);
vector<double> d_uni(26, 0);
// vector<vector<double>> bi(26,vector<double> (26,0)); 
vector<vector<double>> d_bi(26,vector<double> (26, 0));
// double c_uni = 0, c_bi = 0;

void peek(vector<token_T> vt) { for (token_T t: vt) cout << t; }
int ttoi(const token_T& t) { return *(t.c_str())-'a'; }

double get_distance(const token_T& t, const vector<token_T>& prefix) { 
  if (IS_LOCAL) {
    cout << "wrong global variable" << endl;
/*  if (prefix.size()) { return d_bi[ttoi(prefix[0])][ttoi(t)]; }
    else { return d_uni[ttoi(t)]; }
*/}
  if (!prefix.size()) {return plm->lm_score_suffix(t, prefix);} 
  else if (!d_bi[ttoi(prefix[0])][ttoi(t)]) d_bi[ttoi(prefix[0])][ttoi(t)] = plm->lm_score_suffix(t, prefix);
  return d_bi[ttoi(prefix[0])][ttoi(t)];
}

template <typename T>
class Comparator {
  public:
    bool operator()(const T& d1, const T& d2) { return d1 > d2; }
};

class Dnode {
  double distance;
  int original_size;
  int nprefix;
  vector<token_T> unused;
  vector<token_T> used;
  vector<token_T> prefix;

public:
  Dnode(): distance(-1), used({"-1"}) {}
  Dnode(const vector<token_T>& vt):distance(0), nprefix(NPREFIX), original_size(vt.size()), unused(vt) {/* cout << "#DEBUG: I-node constructor" << endl; print(); */}
  Dnode(const Dnode& d, const int& i):
    nprefix(NPREFIX), distance(d.distance+d.to_next(i)), original_size(d.original_size), unused(d.unused), used(d.used), prefix(d.prefix) {
//    cout << "#DEBUG: fringe node constructor" << endl;
      used.push_back(unused[i]);
      if (prefix.size() < nprefix) prefix.push_back(unused[i]);
      else { prefix.erase(prefix.begin()); prefix.push_back(unused[i]); }
      unused.erase(unused.begin()+i); 
  }

  void peek_used() const { cout << "{"; peek(used); cout << "}"; }
  int  size_used() const { return used.size(); }

  double to_next(const int& i) const {
/*  cout <<  "#DEBUG: get_distance() from {"; peek(prefix); cout <<  "} to {" << unused[i] << "}" << endl;
    double dd = get_distance(unused[i], prefix); 
    return dd;
*/  return get_distance(unused[i], prefix);
  }

  void pop(vector<pair<vector<token_T>,double>>& vp, priority_queue<Dnode,vector<Dnode>,Comparator<Dnode>>& pq) const { 
    if (unused.size()) {
//    cout << "#DEBUG: Dnode::pop()" << endl;
      for (int i = 0; i < unused.size(); ++i) {pq.push( Dnode(*this,i)); }
    } else feedback(vp);
  }

  void feedback(vector<pair<vector<token_T>,double>>& vp) const {
//  cout << "#DEBUG: Dnode::feedback()" << endl;
    if (!vp.size()) vp.push_back(make_pair(used, distance));
    else { 
      int i = 0;
      while (i < vp.size()) {
        if (used == vp[i].first) return;
        if (distance >= vp[i].second) i++;
      }
//    if (vp.size() == MAX_ANAGRAM) vp.pop_back();
//    cout << "#DEBUG: word "; peek_used(); cout << " added to vp" << endl;
      vp.insert(vp.begin()+i, make_pair(used, distance));
    }
    for (int i = 0; i < vp.size(); ++i) { cout << "{" << i << ": <"; peek(vp[i].first); cout << ", " << vp[i].second << ">}"; }
    cout << endl; 
  }

  void print() const {
    cout << "distance: " << distance << endl;
    cout << "original_size: " << original_size << endl;
    cout << "nprefix: " << nprefix << endl;
    cout << "unused[]: "; peek(unused); cout << endl;
    cout << "prefix[]: "; peek(prefix); cout << endl;
    cout << "used[]: "; peek(used); cout << endl;
  }

  bool operator>(const Dnode& d) const { return distance > d.distance; } 
  bool operator<(const Dnode& d) const { return distance < d.distance; }
  bool operator==(const Dnode& d) const { return used == d.used; }
  Dnode& operator=(const Dnode& d) { distance = d.distance; unused = d.unused; prefix = d.prefix; used = d.used; return *this; }
};

/*
 * initialize your model. this function will be called before the "train" function
 */
void init() {
  // TODO: do whatever necessary to initialize your model
  cout << "init complete" << endl;
}

/**
 * train your model with a training set
 * \param dataset vector of all training data where each s is in vector of strings
 */
void lm_train(const std::vector<vector<token_T>> &dataset) {
  // TODO: complete this training function
//for (int j = 0; j < 2; ++j){
 
  if (IS_LOCAL) {
    cout << "wrong global variable" << endl;
/*  cout << "training with local dataset" << endl;
    for (int w = 0; w < dataset.size(); ++w) {
      vector<token_T> d = dataset[w];
      int dsize = d.size();
      for (int i = 0; i < dsize; ++i){ uni[ttoi(d[i])]++; if (i > 0) bi[ttoi(d[i-1])][ttoi(d[i])]++; }
      c_uni += dsize; c_bi += (dsize - 1);
    }
    for (int i = 0; i < 26; ++i) { d_uni[i] = -log10( uni[i]/c_uni ); }
    for (int i = 0; i < 26; ++i) { for (int j = 0; j < 26; ++j) { d_bi[j][i] = -log10( bi[j][i]/uni[i] ); } } 
*/} else {
//  cout << "training with language_model" << endl;
    plm = new language_model();
    plm->lm_train(dataset);
  }
    cout << "training complete" << endl;
  
/* string s = "abcdefghijklmnopqrstuvwxyz";
  for (int i = 0; i < 26; ++i) { cout << s[i]  << ": " << get_distance(s.substr(i,1),{}) << endl; }
  for (int i = 0; i < 26; ++i) { for (int j = 0; j < 26; ++j) 
    { cout << s[j] << "|" << s[i] << ": " << get_distance(s.substr(j,1),{s.substr(i,1)}) << endl; }} 
*/
}

/**
 * function to calculate shortest path of given input word using dijkstra's algorithm
 * \param word the input word
 * \return the list of top 5 anagrams paired with their distances
 */
std::vector<std::pair<vector<token_T>, double>> dijkstra(const vector<token_T> &vector_t) {
  // TODO: complete dijkstras algorithm
  vector<pair<vector<token_T>,double>> vp;
//vector<vector<Dnode>> vvd(vector_t.size()+1);
  priority_queue<Dnode,vector<Dnode>,Comparator<Dnode>> pq;
/*string s = "potato"; double sum = 0; vector<token_T> p;
  for (int i = 0; i < s.length(); ++i) { double d = get_distance(s.substr(i,1), p); sum += d; cout << d << endl; if (i > 0) p.erase(p.begin()); p.push_back(s.substr(i,1)); }
  cout << "#TEST: get_distance(""potato""): " << sum << endl;
*/ 
  Dnode I(vector_t); Dnode p = Dnode();
  pq.push(I);
  while ((vp.size() < MAX_ANAGRAM) && (pq.size())) { 
//  cout << "vp.size(): " << vp.size() << ", pq.size(): " << pq.size() << ", pq.top(): "; pq.top().peek_used(); cout << endl;
    Dnode d(pq.top()); pq.pop();
//  vector<Dnode>* vd = &vvd[d.size_used()]; vector<Dnode>::iterator it; it = find(vd->begin(), vd->end(), d);
//  if (it == vd->end()) {
      if (!(d == p)) { d.pop(vp, pq); p = d; }
//  } 
  }
 
  return vp;
}



