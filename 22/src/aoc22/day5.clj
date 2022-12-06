(ns aoc22.day5
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn parse-instruction [s]
  (let [re #"^move (?<count>\d+) from (?<from>\d+) to (?<to>\d+)$"
        m  (re-matcher re s)
        _  (re-find m)]
    {:count (parse-long (.group m "count"))
     :from  (parse-long (.group m "from"))
     :to    (parse-long (.group m "to"))}))

(defn whitespace? [c] (Character/isWhitespace (int c)))

(defn parse [s]
  (let [[stacks instructions] (str/split s #"\n\n")]
    {:stacks       (as-> stacks $
                         (str/split-lines $)
                         ; drop stack labels
                         (drop-last $)
                         (map #(partition 3 4 %) $)
                         (map (fn [line] (map #(nth % 1) line)) $)
                         (partition (count $) (apply interleave $))
                         (map #(drop-while whitespace? %) $))
     :instructions (map parse-instruction (str/split-lines instructions))}))

(defn run [{stacks :stacks, instructions :instructions} get-moved]
  (loop [instructions instructions
         stacks       stacks]
    (if (seq instructions)
      (let [instruction (first instructions)
            {n :count, a :from, b :to} instruction
            moved       (get-moved n (nth stacks (dec a)))]
        (recur (rest instructions)
               (map-indexed
                (fn [idx stack]
                  (cond
                    (= (inc idx) a) (drop n stack)
                    (= (inc idx) b) (concat moved stack)
                    :else stack))
                stacks)))
      (str/join (map first stacks)))))

(defn part1 [input]
  (run input (comp reverse take)))

(defn part2 [input]
  (run input take))

(deftest tests
  (let [sample "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"
        input  (parse sample)]
    (is (= input {:stacks       [[\N \Z] [\D \C \M] [\P]]
                  :instructions [{:count 1, :from 2, :to 1}
                                 {:count 3, :from 1, :to 3}
                                 {:count 2, :from 2, :to 1}
                                 {:count 1, :from 1, :to 2}]}) "parse")
    (is (= (part1 input) "CMZ") "part1")
    (is (= (part2 input) "MCD") "part2")))

(defn -main []
  (let [file  (slurp (io/resource "5.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
