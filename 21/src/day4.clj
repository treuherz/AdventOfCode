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

(defn first-index [pred coll] (first (keep-indexed (fn [i v] (if (pred v) i)) coll)))

(defn empty-marks [n size] (for [_ (range n)] (vec (for [_ (range size)] []))))

(defn bingo [draws boards]
  ;; So we only have to test horizontal lines, we duplicate and transpose the boards
  (let [all-boards (concat boards (map (partial apply mapv vector) boards))]
    (loop [[drawn & rest] draws
           ;; nested vectors of marked numbers
           marked (empty-marks (count all-boards) 5)]
      (if (nil? drawn)
        nil
        (let [marked (mark drawn all-boards marked)]
          ;; get the index of the board for which any row has 5 marks
          (if-let [idx (first-index bingo? marked)]
            {:idx   (mod idx (count boards))                ; we duplicated all the boards, now must undo
             :score (score (nth all-boards idx) (nth marked idx) drawn)}
            (recur rest marked)))))))

(defn part1 [{draws :draws, boards :boards}] (:score (bingo draws boards)))

(defn drop-nth [n coll] (keep-indexed #(if (not= n %1) %2) coll))

(defn part2 [{draws :draws, init-boards :boards}]
  ;; Run the game again and again, removing the winning board each time
  ;; This is inefficient but means I don't need to rip the bingo impl open
  (loop [boards init-boards
         last-win nil]
    (if (empty? boards)
      ;; No more boards, return last win
      (:score last-win)
      (if-let [win (bingo draws boards)]
        (recur (drop-nth (:idx win) boards) win)
        ;; No more winners, return last win
        (:score last-win)))))

(defn -main []
  (let [input (trace "Input" (parse "4.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
