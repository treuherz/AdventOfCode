(ns day4
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace :as trace]))

(defn slurp-resource [file] (slurp (clojure.java.io/resource file)))

(defn split-linegroups [s] (str/split s #"\r?\n\r?\n"))
(defn parse-int [s] (Integer/parseInt s))

(defn parse [file]
  (let [[draws & boards] (split-linegroups (slurp-resource file))]
    {:draws  (as-> draws v
               (str/split v #",")
               (map parse-int v))
     :boards (->> boards
               (mapv str/split-lines)
               (mapv (partial mapv #(str/split (str/triml %) #"\s+")))
               (mapv (partial mapv (partial mapv parse-int))))}))

(defn mark [n boards marked]
  (let [marks (map (partial map (partial filter (partial = n))) boards)]
    (mapv (partial mapv #(apply conj %1 %2)) marked marks)))

(defn score [board marks n]
  (let [sum-all (reduce + (flatten board))
        sum-marks (reduce + (flatten marks))]
    (* n (- sum-all sum-marks))))

(defn bingo? [marks] (boolean (some #(= 5 (count %)) marks)))

(defn positions [pred coll] (keep-indexed (fn [i v] (when (pred v) i)) coll))

(defn empty-marks [n size] (for [_ (range n)] (vec (for [_ (range size)] []))))

(defn part1 [{draws :draws, boards :boards}]
  ;; So we only have to test horizontal lines, we duplicate and transpose the boards
  (let [all-boards (concat boards (map (partial apply mapv vector) boards))]
    (loop [[drawn & rest] draws
           ;; nested vectors of marked numbers
           marked (empty-marks (count boards) 5)]
      (let [marked (mark drawn all-boards marked)]
        ;; get the index of the board for which any row has 5 marks
        (if-let [idx (first (positions bingo? marked))]
          (score (nth all-boards idx) (nth marked idx) drawn)
          (recur rest marked))))))

(defn part2 [input] "-")

(defn -main []
  (let [input (trace "Input" (parse "4.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
