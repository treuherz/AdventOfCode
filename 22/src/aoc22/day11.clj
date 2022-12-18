(ns aoc22.day11
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn split-linegroups [s] (str/split s #"\r?\n\r?\n"))

(defn parse-monkey [s]
  (let [re #"Monkey (?<monkey>\d+):\n\s+Starting items: (?<items>[\d, ]+)\n\s+Operation: new = old (?<op>[*+]) (?<operand>[\w]+)\n\s+Test: divisible by (?<div>\d+)\n\s+If true: throw to monkey (?<iftrue>\d+)\n\s+If false: throw to monkey (?<iffalse>\d+)"
        m  (re-matcher re s)]
    (when (re-find m)
      {:monkey   (parse-long (.group m "monkey"))
       :items    (mapv parse-long (str/split (.group m "items") #", "))
       :op       (condp = (.group m "op")
                   "*" *
                   "+" +)
       :operand  (if-some [n (parse-long (.group m "operand"))] n :old)
       :div      (parse-long (.group m "div"))
       :if-true  (parse-long (.group m "iftrue"))
       :if-false (parse-long (.group m "iffalse"))})))

(defn parse [s]
  (as-> s $
        (split-linegroups $)
        (map parse-monkey $)
        (into {} (for [m $] [(:monkey m) m]))))

(defn monkey-modulo [monkeys]
  (reduce * (map :div (vals monkeys))))

(defn monkey-business [monkeys]
  (->> monkeys
       vals
       (map :n)
       sort
       reverse
       (take 2)
       (reduce *)))

(defn inspect [{op :op, operand :operand, divisor :div, if-true :if-true, if-false :if-false} old relief modulo]
  (let [operand  (if (= operand :old) old operand)
        worried  (op old operand)
        relieved (relief (mod worried modulo))
        test     (zero? (mod relieved divisor))
        target   (if test if-true if-false)]
    [target relieved]))

(defn run [input rounds relief]
  (let [modulo (monkey-modulo input)]
    (loop [round-idx  0
           monkey-idx 0
           monkeys    (into {} (for [[m monkey] input] [m (assoc monkey :n 0)]))]
      (if (< round-idx rounds)
        (if (< monkey-idx (count monkeys))
          (let [monkey         (get monkeys monkey-idx)
                inspected      (for [item (:items monkey)] (inspect monkey item relief modulo))
                target-monkeys (map first inspected)
                target-items   (into {} (for [m target-monkeys] [m (map second (filter #(= (first %) m) inspected))]))
                n              (+ (:n monkey) (count inspected))
                monkeys        (into {} (for [[m monkey] monkeys
                                              :let [items     (:items monkey)
                                                    receiving (get target-items m)]]
                                          [m
                                           (cond
                                             (= monkey-idx m) (assoc monkey :n n :items [])
                                             receiving (assoc monkey :items (concat items receiving))
                                             :else monkey)]))]
            (recur round-idx (inc monkey-idx) monkeys))
          (recur (inc round-idx) 0 monkeys))
        (monkey-business monkeys)))))

(defn part1 [input]
  (run input 20 #(quot % 3)))

(defn part2 [input]
  (run input 10000 identity))

(deftest tests
  (let [sample1 "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1"
        input1  (parse sample1)]
    (is (= (get input1 0) {:monkey 0, :items [79, 98], :op *, :operand 19, :div 23, :if-true 2, :if-false 3}) "parse sample1")
    (is (= (monkey-business {0 {:n 101}, 1 {:n 95}, 2 {:n 7}, 3 {:n 105}}) 10605))
    (is (= (part1 input1) 10605) "part1")
    (is (= (part2 input1) 2713310158) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "11.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
