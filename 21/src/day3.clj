(ns day3
  (:require [clojure.string :as str]))

(defn bit-string-to-ints [string]
  (map
    ;; type hint to specify which overload of Character/digit we're asking for
    (fn [^Character ch] (Character/digit ch 10))
    (seq string)))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (map bit-string-to-ints)))

(defn most-common-bits [input]
  (->> input
    (apply mapv +)
    (map #(>= % (/ (count input) 2)))
    (map {false 0 true 1})))

(defn least-common-bits [input]
  (map {1 0 0 1} (most-common-bits input)))

(defn bits-to-int [bits] (Integer/parseInt (apply str bits) 2))

(defn gamma [input]
  (bits-to-int (most-common-bits input)))

(defn epsilon [input]
  (bits-to-int (least-common-bits input)))

(defn part1 [input] (* (epsilon input) (gamma input)))

(defn filter* [list maskf n]
  (let [mask (maskf list)]
    (filter #(= (nth % n) (nth mask n)) list)))

(defn life-support [input maskf]
  (loop [list input n 0]
    (if (= (count list) 1)
      (first list)
      (recur (filter* list maskf n) (+ n 1)))))

(defn oxygen-generator [input]
  (bits-to-int (life-support input most-common-bits)))

(defn co2-scrubber [input]
  (bits-to-int (life-support input least-common-bits)))

(defn part2 [input] (* (oxygen-generator input) (co2-scrubber input)))

(defn -main []
  (let [input (parse "3.txt")]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
