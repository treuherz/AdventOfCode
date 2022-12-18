(ns aoc22.day10
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(def arg-keys
  (->> (iterate inc 1)
       (map str)
       (map keyword)))

(defn parse-instruction [[op & args]]
  (apply merge {:op (keyword op)} (zipmap arg-keys (map parse-long args))))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(str/split % #" "))
       (map parse-instruction)))

(defn record-if [at cycle coll x]
  (if (contains? at cycle)
    (conj coll x)
    coll))

(defn instruction-duration [instruction]
  (condp = (:op instruction)
    :noop 1
    :addx 2))

(defn record [recorded record-cycle cycle duration x]
  (loop [n        0
         recorded recorded]
    (if (< n duration)
      (let [cycle         (+ cycle n)
            should-record (record-cycle cycle)
            recorded      (if should-record (conj recorded (* cycle x)) recorded)]
        (recur (inc n) recorded))
      recorded)))


(defn run [instructions record-cycle]
  (loop [cycle        1
         x            1
         instructions instructions
         recorded     []]
    (if-let [[instruction & instructions] (seq instructions)]
      (let [duration   (instruction-duration instruction)
            next-cycle (+ cycle duration)
            recorded   (record recorded record-cycle cycle duration x)]
        (condp = (:op instruction)
          :noop (recur next-cycle x instructions recorded)
          :addx (recur next-cycle (+ x (:1 instruction)) instructions recorded)))
      recorded)))

(defn part1 [input]
  (reduce + (run input #(= (mod % 40) 20))))

(defn add-pixels [screen start-cycle duration x]
  (loop [cycle  start-cycle
         screen screen]
    (if (< cycle (+ start-cycle duration))
      (let [sprite  (set [(dec x) x (inc x)])
            lit     (contains? sprite (mod (dec cycle) 40))
            pixel   (if lit \# \.)
            screen  (str screen pixel)]
        (recur (inc cycle) screen))
      screen)))

(defn apply-instruction [{op :op, arg1 :1} x]
  (condp = op
    :noop x
    :addx (+ x arg1)))

(defn part2 [input]
  (loop [cycle        1
         x            1
         instructions input
         screen       ""]
    (if-let [[i & instructions] (seq instructions)]
      (let [duration   (instruction-duration i)
            next-cycle (+ cycle duration)
            screen     (add-pixels screen cycle duration x)
            x          (apply-instruction i x)]
        (recur next-cycle x instructions screen))
      (str/join "\n" (map str/join (partition 40 screen))))))

(deftest tests
  (let [sample1 "noop\naddx 3\naddx -5"
        sample2 "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop"
        input1  (parse sample1)
        input2  (parse sample2)]
    (is (= input1 [{:op :noop} {:op :addx, :1 3} {:op :addx, :1 -5}]) "parse sample1")
    (is (= (part1 input2) 13140) "part1")
    (is (= (part2 input2) "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....") "part2")))

(defn -main []
  (let [file  (slurp (io/resource "10.txt"))
        input (parse file)]
    (println "Part 1:" (part1 input))
    (println "Part 2:")
    (println (part2 input))))
