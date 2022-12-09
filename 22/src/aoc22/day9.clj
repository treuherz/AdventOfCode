(ns aoc22.day9
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]
   [clojure.tools.trace :refer [trace deftrace]]))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(str/split % #" "))
       (map (fn [[dir n]] [(keyword dir) (parse-long n)]))))

(defn move-orth [[x y] dir]
  (condp = dir
    :U [x (inc y)]
    :D [x (dec y)]
    :L [(dec x) y]
    :R [(inc x) y]))

(defn dec-instruction [[dir n]] [dir (dec n)])

(defn catch-up [[x y] [del-x del-y]]
  (condp = [del-x del-y]
    [0 2] [x (inc y)]
    [1 2] [(inc x) (inc y)]
    [2 2] [(inc x) (inc y)]
    [2 1] [(inc x) (inc y)]
    [2 0] [(inc x) y]
    [2 -1] [(inc x) (dec y)]
    [2 -2] [(inc x) (dec y)]
    [1 -2] [(inc x) (dec y)]
    [0 -2] [x (dec y)]
    [-1 -2] [(dec x) (dec y)]
    [-2 -2] [(dec x) (dec y)]
    [-2 -1] [(dec x) (dec y)]
    [-2 0] [(dec x) y]
    [-2 1] [(dec x) (inc y)]
    [-2 2] [(dec x) (inc y)]
    [-1 2] [(dec x) (inc y)]))


(defn part1 [input]
  (loop [h-pos        [0 0]
         t-pos        [0 0]
         instruction  (first input)
         instructions (rest input)
         t-visited    #{[0 0]}]
    (if (zero? (second instruction))
      (if-let [[instruction & instructions] (seq instructions)]
        (recur h-pos t-pos instruction instructions t-visited)
        (count t-visited)) ; exit
      (let [h-pos       (move-orth h-pos (first instruction))
            delta       (map - h-pos t-pos)
            too-far     (some #(= 2 (abs %)) delta)
            t-pos       (if too-far (catch-up t-pos delta) t-pos)
            instruction (dec-instruction instruction)
            t-visited   (conj t-visited t-pos)]
        (recur h-pos t-pos instruction instructions t-visited)))))

(defn move-along-rope [[h-pos & rest-pos] dir]
  (let [h-pos     (move-orth h-pos dir)
        positions (vec (cons h-pos rest-pos))]
    (loop [positions positions
           n         1]
      (if (< n (count positions))
        (let [t-pos   (nth positions n)
              h-pos   (nth positions (dec n))
              delta   (map - h-pos t-pos)
              too-far (some #(= 2 (abs %)) delta)
              t-pos   (if too-far (catch-up t-pos delta) t-pos)]
          (recur (assoc positions n t-pos) (inc n)))
        positions))))

(defn part2 [input]
  (loop [positions    (vec (repeat 10 [0 0]))
         instruction  (first input)
         instructions (rest input)
         t-visited    #{[0 0]}]
    (if (zero? (second instruction))
      (if-let [[instruction & instructions] (seq instructions)]
        (recur positions instruction instructions t-visited)
        (count t-visited))
      (let [positions   (move-along-rope positions (first instruction))
            t-pos       (last positions)
            instruction (dec-instruction instruction)
            t-visited   (conj t-visited t-pos)]
        (recur positions instruction instructions t-visited)))))


(deftest tests
  (let [sample1 "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"
        sample2 "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"
        input1  (parse sample1)
        input2  (parse sample2)]
    (is (= input1 [[:R 4] [:U 4] [:L 3] [:D 1] [:R 4] [:D 1] [:L 5] [:R 2]]) "parse")
    (is (= (part1 input1) 13) "part1")
    (is (= (part2 input1) 1) "part2 sample1")
    (is (= (part2 input2) 36) "part2 sample2")))

(defn -main []
  (let [file  (slurp (io/resource "9.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
