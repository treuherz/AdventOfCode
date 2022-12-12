(ns aoc22.day8
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(str/split % #""))
       (map #(map parse-long %))))

(defn deep-nth [coll & ns]
  (loop [coll coll
         ns   ns]
    (if-let [[n & ns] (seq ns)]
      (recur (nth coll n) ns)
      coll)))

;; indices of cells between [y x] and the edge, going leftwards, rightwards, upwards, downards
(defn lines-of-sight [y x y-count x-count]
  [(map vector (repeat y) (reverse (range 0 x)))
   (map vector (repeat y) (range (inc x) x-count))
   (map vector (reverse (range 0 y)) (repeat x))
   (map vector (range (inc y) y-count) (repeat x))])

(defn count-true [coll] (count (filter identity coll)))

(defn any-true? [coll] (some identity coll))

(defn part1 [input]
  (let [y-count (count input)
        x-count (count (first input))]
    (count-true
     (for [y (range y-count)
           x (range x-count)
           :let [tree (deep-nth input y x)]]
       (any-true? (for [line-indices (lines-of-sight y x y-count x-count)
                        :let [trees-in-line (for [[y x] line-indices] (deep-nth input y x))]]
                    (or
                     ; the tree is on the margin, there's nothing in line-of-sight this way
                     (empty? line-indices)
                     ; every tree in the line-of-sight is shorter than this tree
                     (every? #(> tree %) trees-in-line))))))))

(defn viewing-distance [tree-height trees-in-line]
  (loop [n     0
         trees trees-in-line]
    (if-let [[next-tree & trees] (seq trees)]
      (if (> tree-height next-tree)
        (recur (inc n) trees)
        (recur (inc n) ()))
      n)))

(defn part2 [input]
  (let [y-count (count input)
        x-count (count (first input))]
    (apply max
           (for [y (range y-count)
                 x (range x-count)
                 :let [tree (deep-nth input y x)]]
             (reduce * (for [line-indices (lines-of-sight y x y-count x-count)
                             :let [trees-in-line (for [[y x] line-indices] (deep-nth input y x))]]
                         (if (empty? line-indices)
                           0
                           (viewing-distance tree trees-in-line))))))))

(deftest tests
  (let [sample "30373\n25512\n65332\n33549\n35390"
        input  (parse sample)]
    (is (= input [[3 0 3 7 3] [2 5 5 1 2] [6 5 3 3 2] [3 3 5 4 9] [3 5 3 9 0]]) "parse")
    (is (= (deep-nth [1 [1 [2]]] 0) 1))
    (is (= (deep-nth [2 [1 [2]]] 1 0) 2))
    (is (= (deep-nth [3 [1 [2]]] 1 1 0) 3))
    (is (= (lines-of-sight 2 2 5 5)
           [[[2 1] [2 0]]
            [[2 3] [2 4]]
            [[1 2] [0 2]]
            [[3 2] [4 2]]]))
    (is (= (part1 input) 21) "part1")
    (is (= (part2 input) 8) "part2")))

(defn -main []
  (let [file  (slurp (io/resource "8.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
