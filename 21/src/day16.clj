(ns day16
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace])
  (:require [clojure.zip :as zip]))

(defn hex->bits [h]
  (let [pad-len (- (Integer/numberOfLeadingZeros h) (- 32 4))
        padding (repeat pad-len \0)
        binary  (Integer/toBinaryString h)]
    (concat padding binary)))

(defn bits->int [b] (Integer/parseInt (apply str b) 2))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (map #(Character/digit ^char % 16))
    (mapcat hex->bits)))

(defn round-up [offset] (- 4 (mod offset 4)))

(defn literal-data [input offset]
  (loop [b      []
         input  input
         offset offset
         done   false]
    (trace "b" b)
    (trace "input" input)
    (trace "offset" offset)
    (trace "done" done)
    (if done
      [(bits->int b) (drop (- 4 (mod offset 4)) input)]
      (recur
        (concat b (rest (take 5 input)))
        (drop 5 input)
        (+ offset 5)
        (= (first input) \0)))))

(defn packet [input]
  (loop [structure  []
         path  [0]
         input input]
    (if (empty? input)
      structure
      (let [v (bits->int (take 3 input))
            t (bits->int (take 3 (drop 3 input)))]
        (if (= t 4)
          (let [[value input] (literal-data (drop 6 input) 6)
                structure (-> structure
                       (assoc-in (conj path :v) v)
                       (assoc-in (conj path :t) t)
                       (assoc-in (conj path :value) value))
                ;; increment index at tip of path
                path (update path (dec (count path)) inc)]
            (recur structure path input))
          (case (nth 7 input)
            \0 (let [len (bits->int (take 15 (drop 7 input)))
                     structure (-> structure
                                 (assoc-in (conj path :v) v)
                                 (assoc-in (conj path :t) t)
                                 (assoc-in (conj path :nested) (packet (take len input))))
                     path (update path (dec (count path)) inc)]
                 (recur structure path (drop (+ 7 15 (round-up len) input)))
            \1 (let [num (bits->int (take 11 (drop 7 input)))
                     structure (-> structure
                                 (assoc-in (conj path :v) v)
                                 (assoc-in (conj path :t) t)
                                 (assoc-in (conj path :nested 0) (packet ))
                                 )])))))))


(defn part1 [input])

(defn part2 [input])

(defn -main []
  (let [input (trace (parse "16_sample.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
