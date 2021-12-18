(ns day14
  (:require [clojure.string :as str])
  (:use [clojure.tools.trace]))

(defn parse-rule [s]
  (let [re #"^(?<pair>[A-Z][A-Z]) -\> (?<insert>[A-Z])$"
        m  (re-matcher re s)]
    (do
      (re-find m)
      {:left   (first (.group m "pair"))
       :right  (second (.group m "pair"))
       :insert (first (.group m "insert"))})))

(defn parse [file]
  (->> file
    (clojure.java.io/resource)
    (slurp)
    (str/split-lines)
    (#(identity {:start (first %)
                 :rules (map parse-rule (drop 2 %))}))))

(defn insertions [s {:keys [left right insert]}]
  (keep-indexed
    (fn [idx [l r]]
      (when (and (= left l) (= right r))
        {:insert insert, :at (+ idx 1)}))
    (partition 2 1 s)))

(defn apply-rules [s rules]
  (let [insertions (mapcat #(insertions s %) rules)]
    ((reduce
       (fn [{idxs :idxs, s :str} {c :insert, idx :at}]
         ;; Adjust index to account for previous insertions
         (let [idx* (+ idx (count (filter #(>= idx %) idxs)))]
           {:idxs (conj idxs idx), :str (str (subs s 0 idx*) c (subs s idx*))}))
       {:idxs [], :str s}
       insertions)
     :str)))

(defn part1 [{:keys [start rules]}]
  (let [end       (nth (iterate #(apply-rules % rules) start) 10)
        freqs     (frequencies end)
        max-count (apply max (vals freqs))
        min-count (apply min (vals freqs))]
    (- max-count min-count)))

(defn part2 [input])

(defn -main []
  (let [input (trace (parse "14.txt"))]
    (println "Part 1:" (part1 input))
    (println "Part 2:" (part2 input))))
