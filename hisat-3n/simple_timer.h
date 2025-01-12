#pragma once

#include <cstdlib>
#include <ctime>
#include <iostream>
#include <sstream>
#include <string>
#include <sys/time.h>
#include <unordered_map>

using namespace std;

class SimpleTimer {
public:
  SimpleTimer() {
    whole_program.begin_time = get_current_time();
    whole_program.count = 1;
  }
  static void begin(const std::string &func_name = "Anonymous Function") {
    std::unordered_map<std::string, Data>::iterator it =
        instance().datas.find(func_name);
    if (it == instance().datas.end()) {
      instance().datas.insert(pair<std::string, Data>(func_name, Data()));
      it = instance().datas.find(func_name);
    }
    it->second.begin_time = get_current_time();
  }

  static void end(const std::string &func_name = "Anonymous Function") {
    std::unordered_map<std::string, Data>::iterator it =
        instance().datas.find(func_name);
    if (it != instance().datas.end()) {
      it->second.total_run_time += get_current_time() - it->second.begin_time;
      it->second.count++;
    } else {
      std::cerr << "Error: This function '" << func_name << "' is not defined"
                << std::endl;
      system("pause");
    }
  }

  static std::string summery(void) {
    SimpleTimer::instance().whole_program.total_run_time =
        get_current_time() - SimpleTimer::instance().whole_program.begin_time;

    std::stringstream ss;

    static const string func_name = "Function Name";
    static const int func_name_size = max((int)func_name.length(), 30);

    static const string total_time = "Total";
    static const int total_time_size = max((int)total_time.length(), 25);

    static const string called_times = "Be Called";
    const int called_times_size = max((int)called_times.length(), 15);

    static const string percentage = "Percentage";
    const int percentage_size = max((int)percentage.length(), 15);

    ss << "+--------------------" << endl;
    ss << "| Profiling Summery ..." << endl;
    ss << "+---------------------------------------" << endl;
    ss << "| ";
    ss.width(func_name_size);
    ss << std::left << func_name;
    ss << " |";
    ss.width(total_time_size);
    ss << std::right << total_time;
    ss << " |";
    ss.width(called_times_size);
    ss << std::right << called_times;
    ss << " |";
    ss.width(percentage_size);
    ss << std::right << percentage;
    ss << " |" << std::endl;

    double tatol_running_time =
        SimpleTimer::instance().whole_program.total_run_time;

    ss << "| ";
    ss.width(func_name_size);
    ss << std::left << "Total Run Time" << " |";
    ss.width(total_time_size - sizeof("ms"));
    ss << std::right << tatol_running_time << " ms |";
    ss.width(called_times_size - sizeof("times"));
    ss << std::right << 1 << " times |";
    ss.width(percentage_size - sizeof("%"));
    ss << std::right << 100.000 << " % |" << endl;

    std::unordered_map<std::string, Data>::iterator it;
    for (it = instance().datas.begin(); it != instance().datas.end(); it++) {
      ss << "| ";
      ss.width(func_name_size);
      ss << std::left << it->first << " |";
      ss.width(total_time_size - sizeof("ms"));
      ss << std::right << it->second.total_run_time << " ms |";
      ss.width(called_times_size - sizeof("times"));
      ss << std::right << it->second.count << " times |";
      ss.width(percentage_size - sizeof("%"));
      ss << std::right
         << int(100000 * it->second.total_run_time / tatol_running_time) /
                1000.0
         << " % |" << endl;
    }

    return ss.str();
  }

private:
  // Private Functions
  inline static SimpleTimer &instance() {
    static SimpleTimer t;
    return t;
  }

  // Data Structures
  struct Data {
    double total_run_time, begin_time;
    int count;
    Data() : total_run_time(0.0), begin_time(0.0), count(0) {}
  };

  // running time data for the whole program
  Data whole_program;
  std::unordered_map<std::string, Data> datas;

  static double get_current_time() {
    static timeval t;
    gettimeofday(&t, NULL);
    return 1.0 * t.tv_sec * 1000.0     // sec to ms
           + 1.0 * t.tv_usec / 1000.0; // us to ms
  }
};

// New RAII wrapper class
class ScopedTimer {
public:
  explicit ScopedTimer(const std::string &name = "Anonymous Function")
      : function_name(name) {
    SimpleTimer::begin(function_name);
  }

  // Non-copyable
  ScopedTimer(const ScopedTimer &) = delete;
  ScopedTimer &operator=(const ScopedTimer &) = delete;

  ~ScopedTimer() { SimpleTimer::end(function_name); }

private:
  std::string function_name;
};
