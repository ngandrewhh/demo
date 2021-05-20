//
// Created by Dekai WU and Serkan KUMYOL on 20190308.
//

#ifndef _LANGUAGE_MODEL_HPP_
#define _LANGUAGE_MODEL_HPP_

#include <unordered_set>
#include <vector>
#include <string>

/** token_T is designed for holding only one character.
 * The reason token_T is defined as std::string, instead of char,
 * is so that the character can be a Unicode character instead of just an ASCII character.
 * The Unicode character is stored in a std::string by encoding it using UTF-8.
 * */
using token_T = std::string;

/**
 * Semiring is the way you choose how to measure distance between tokens.
 */
enum semiring_t {NEG_LOG_10};

using std::vector;
/**
 * a model that takes a token and predicts a label
 */
class language_model {
    std::string m_id;
public:
  language_model();
  language_model(const language_model&) = delete;
  language_model(language_model&&) noexcept = default;
  language_model &operator=(const language_model&) = delete;
  language_model &operator=(language_model&&) noexcept = default;

/**
 * train your model with a training set
 * \param dataset vector of all training data where each s is in vector of strings
 */
  void lm_train(const vector<vector<token_T>> &words);

    /**
     * given compute d(e1 | e0)
     * \param token e1, the token to score
     * \param prefix the context of the token. in the bigram case, it contains only the preceding token e0
     * \param semiring the number domain. your only choice is negative log 10 domain
     * \return score value
     */
   double lm_score_suffix(const token_T &token, const std::vector<token_T> &prefix = {}, semiring_t semiring=NEG_LOG_10) const;

};

// below are utility functions that you don't need in assignment 1

/**
 * Takes vector of pairs of token and scores and prints.
 * \param top5anagrams
 */
void print_top5anagrams(const std::vector<std::pair<vector<token_T>, double>> &top5anagrams);



std::vector<std::vector<token_T>> read_dataset(std::istream& is);

#endif
