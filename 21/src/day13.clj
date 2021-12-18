(ns day13
  (:require [clojure.string :as str])
  (:require [clojure.set :as set])
  (:use [clojure.tools.trace]))

(defn parse-int [s] (Integer/parseInt s))
(defn digit? [c] (Character/isDigit ^char c))

(defn parse-dots [lines]
  (->> lines
    (map #(str/split % #","))
    (map (partial mapv parse-int))))

(defn parse-folds [lines]
  (let [re #"fold along (?<xy>[xy])=(?<n>\d+)"]
    (for [line lines
          :let [m (re-matcher re line)]]
      (do
        (re-find m)
        {:axis (keyword (.group m "xy"))
         :n    (parse-int (.group m "n"))}))))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    ((fn [in] {:dots  (parse-dots (take-while #(and (not (empty? %)) digit? (first %)) in))
               :folds (parse-folds (drop-while #(or (empty? %) (digit? (first %))) in))}))))

(defn fold [[x y] {axis :axis, n :n}]
  (case axis
    :x [(- n (Math/abs (- x n))) y]
    :y [x (- n (Math/abs (- y n)))]))

(defn part1 [input]
  (->> (:dots input)
    (map #(fold % (first (:folds input))))
    (set)
    (count)))

(defn part2 [{:keys [folds, dots]}]
  (let [positions (set
                    (map #(reduce (partial fold) % folds) dots))
        max-x     (:n (last (filter #(= (:axis %) :x) folds)))
        max-y     (:n (last (filter #(= (:axis %) :y) folds)))]
    (with-out-str
      (doseq [y (range 0 max-y)
              x (range 0 max-x)]
        (when (zero? x) (println))
        (if (contains? positions [x y])
          (print \#)
          (print \.))))))

(defn -main []
  (let [input (trace (parse "13.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
