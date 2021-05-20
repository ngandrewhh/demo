//
// Created by Dekai WU and Serkan Kumyol on 20190307.
//

#include <iostream>
#include <fstream>
#include "assignment.hpp"

using namespace std;
int main(int argc, char* argv[]) {
  if (argc < 2)
    throw std::runtime_error("Please enter an argument.");
  string input_word = argv[1];
  vector<string> input;
  for (const auto& token: input_word) {
    input.emplace_back(1, token);
  }
  ifstream train_ifs("traindata.xml");

  auto training_words = read_dataset(train_ifs);
  train_ifs.close();

  cout << "initializing" << endl;
  init();
  auto lm = language_model();
  cout << "training model" << endl;
  lm_train(training_words);

  cout << "running dijkstra" << endl;
  auto top5anagrams = dijkstra(input);//run dijkstras algorithm

  print_top5anagrams(top5anagrams);

  return 0;
}
