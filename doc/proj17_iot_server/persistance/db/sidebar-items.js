initSidebarItems({"fn":[["draw_credits_for_order","Decrements the credit field for orders row matching id"],["find_eligible_videos_by_interest","Returns all elligible videos for the interests contained in the Vec interests with interest_id's. Only returns videos that are payed for, and matches one of the interests given"],["get_advertisement_video_by_id","Returns an AdvertVideo if exists"],["get_display_by_id","Returns Display if exists by id"],["get_display_location","Returns the location if exists of the display by id"],["get_interests_at_location","Returns the aggregated weight of all the interests for trackers in this location and turns into a reverse weight sorted tuple of (interest, weight)."],["get_order_by_id","Returns an Order if exists"],["get_receiver_by_id","Returns a Receiver if exists by id"],["get_tracker_by_id","Returns an Tracker if exists by id"],["insert_played_video","Inserts a played_video row in the database"],["register_tracker_to_receiver","Sets the location of a tracker by id"],["unregister_tracker","Sets the location of a tracker to null of exists by id"]],"struct":[["DB",""],["Dbconn","Database connection pool structure"]],"trait":[["PrintErr",""]]});