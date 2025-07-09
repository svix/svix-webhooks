package com.svix.test;

import static org.junit.Assert.*;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.models.*;
import java.util.HashMap;
import org.junit.Test;

public class ModelTests {
  @Test
  public void toJsonWorks() throws JsonProcessingException {
    DashboardAccessOut model = new DashboardAccessOut().token("asd");
    assertEquals("{\"token\":\"asd\"}", model.toJson());
  }

  @Test
  public void fromJsonWorks() throws JsonProcessingException {
    DashboardAccessOut expectedModel = new DashboardAccessOut().token("asd");
    DashboardAccessOut model = DashboardAccessOut.fromJson("{\"token\":\"asd\"}");
    assertEquals(expectedModel, model);
  }

  @Test
  public void structEnumWithNoExtraFields() throws JsonProcessingException {
    String jsonString =
        "{\"metadata\":{},\"name\":\"mendy\",\"uid\":\"very"
            + " unique\",\"type\":\"generic-webhook\",\"config\":{}}";
    IngestSourceIn sourceIn =
        new IngestSourceIn(
            new HashMap<>(), "mendy", "very unique", new IngestSourceInConfig.GenericWebhook());

    assertEquals(jsonString, sourceIn.toJson());
    assertEquals(IngestSourceIn.fromJson(jsonString), sourceIn);
  }

  @Test
  public void structEnumWithFields() throws JsonProcessingException {
    String jsonString =
        "{\"metadata\":{},\"name\":\"name\",\"uid\":\"uuiidd\",\"type\":\"cron\",\"config\":{\"contentType\":\"asd\",\"payload\":\"cool\",\"schedule\":\"*"
            + " * * * *\"}}";
    IngestSourceIn sourceIn =
        new IngestSourceIn(
            new HashMap<>(),
            "name",
            "uuiidd",
            new IngestSourceInConfig.Cron(
                new CronConfig().contentType("asd").payload("cool").schedule("* * * * *")));
    assertEquals(jsonString, sourceIn.toJson());
    assertEquals(IngestSourceIn.fromJson(jsonString), sourceIn);
  }

  @Test
  public void readStructEnumField() throws JsonProcessingException {
    String jsonString =
        "{\"name\":\"name\",\"uid\":\"uuiidd\",\"type\":\"cron\",\"config\":{\"contentType\":\"asd\",\"payload\":\"cool\",\"schedule\":\"*"
            + " * * * *\"}}";
    IngestSourceIn sourceIn = IngestSourceIn.fromJson(jsonString);
    assertTrue(sourceIn.getConfig() instanceof IngestSourceInConfig.Cron);
    assertEquals(
        "asd", ((IngestSourceInConfig.Cron) sourceIn.getConfig()).getCron().getContentType());
  }

  @Test
  public void opWebhookModels() throws JsonProcessingException {
    String jsonString =
        "{\"data\":{\"data\":{\"appStats\":[{\"appId\":\"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",\"appUid\":nu"
            + "ll,\"messageDestinations\":343}]},\"status\":\"finished\",\"task\":\"application.stat"
            + "s\",\"taskId\":\"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2\"},\"type\":\"background_task.finished\"}";
    BackgroundTaskFinishedEvent backgroundTask = BackgroundTaskFinishedEvent.fromJson(jsonString);

    assertEquals(jsonString, backgroundTask.toJson());
    assertEquals(BackgroundTaskFinishedEvent.fromJson(jsonString), backgroundTask);
  }
}
