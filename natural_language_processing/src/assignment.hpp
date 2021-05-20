//
// Created by Dekai WU and Serkan Kumyol on 20190307.
//

#ifndef COMP3211_2019Q1_A1_STUB_ASSIGNMENT_HPP
#define COMP3211_2019Q1_A1_STUB_ASSIGNMENT_HPP
#include "language_model.hpp"

extern const char* STUDENT_ID;

void init();

void lm_train(const vector<vector<token_T>>& dataset);

std::vector<std::pair<vector<token_T >, double>> dijkstra(const vector<token_T>& tokens);

#endif
